use leptos::prelude::*;
use leptos_fluent::move_tr;
use serde::{Deserialize, Serialize};

use crate::icons::Icon;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct ReleaseNotes {
    pub(crate) tag_name: String,
    pub(crate) name: String,
    pub(crate) published_at: String,
    pub(crate) body: String,
}

impl ReleaseNotes {
    fn fallback() -> Self {
        Self {
            tag_name: String::new(),
            name: "Kopuz release notes".into(),
            published_at: String::new(),
            body: "Could not load release notes.".into(),
        }
    }

    /// Version without the leading `v`, empty when the fetch fell back.
    pub(crate) fn version(&self) -> &str {
        self.tag_name.trim_start_matches('v')
    }
}

#[cfg(feature = "ssr")]
#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    name: Option<String>,
    published_at: Option<String>,
    body: Option<String>,
}

#[cfg(feature = "ssr")]
mod cache {
    use super::ReleaseNotes;
    use std::sync::{Mutex, OnceLock};
    use std::time::{Duration, Instant};

    const TTL: Duration = Duration::from_secs(600);

    #[allow(clippy::type_complexity)]
    static CACHE: OnceLock<Mutex<Option<(Instant, ReleaseNotes)>>> = OnceLock::new();

    fn cell() -> &'static Mutex<Option<(Instant, ReleaseNotes)>> {
        CACHE.get_or_init(|| Mutex::new(None))
    }

    pub(super) fn get() -> Option<ReleaseNotes> {
        let guard = cell().lock().ok()?;
        guard
            .as_ref()
            .filter(|(at, _)| at.elapsed() < TTL)
            .map(|(_, release)| release.clone())
    }

    pub(super) fn put(release: &ReleaseNotes) {
        if let Ok(mut guard) = cell().lock() {
            *guard = Some((Instant::now(), release.clone()));
        }
    }
}

/// The shelf and the player bar render on every page, so an uncached fetch
/// would spend the anonymous GitHub rate limit within an hour of traffic.
pub(crate) async fn fetch_latest_release() -> ReleaseNotes {
    #[cfg(feature = "ssr")]
    {
        if let Some(cached) = cache::get() {
            return cached;
        }

        let client = reqwest::Client::builder()
            .user_agent("kopuz-website/1.0")
            .build();
        if let Ok(client) = client {
            let response = client
                .get("https://api.github.com/repos/Kopuz-org/kopuz/releases/latest")
                .header("Accept", "application/vnd.github+json")
                .send()
                .await;
            if let Ok(response) = response {
                if let Ok(body) = response.text().await {
                    if let Ok(release) = serde_json::from_str::<GitHubRelease>(&body) {
                        let release = ReleaseNotes {
                            name: release.name.unwrap_or_else(|| release.tag_name.clone()),
                            tag_name: release.tag_name,
                            published_at: release.published_at.unwrap_or_default(),
                            body: release.body.unwrap_or_default(),
                        };
                        cache::put(&release);
                        return release;
                    }
                }
            }
        }
    }

    ReleaseNotes::fallback()
}

/// Seeded once per page from `provide_site_theme` so the hero, the shelf and
/// the release notes share a single fetch.
pub(crate) fn provide_latest_release() {
    let release = Resource::new(|| (), |_| async move { fetch_latest_release().await });
    provide_context(release);
}

pub(crate) fn use_latest_release() -> Resource<ReleaseNotes> {
    use_context::<Resource<ReleaseNotes>>().unwrap_or_else(|| {
        Resource::new(|| (), |_| async move { fetch_latest_release().await })
    })
}

#[derive(Debug, PartialEq, Eq)]
enum ReleaseInline {
    Text(String),
    Strong(String),
    Code(String),
    Link { label: String, href: String },
}

#[derive(Debug, PartialEq, Eq)]
enum ReleaseBlock {
    Heading2(Vec<ReleaseInline>),
    Heading3(Vec<ReleaseInline>),
    Heading4(Vec<ReleaseInline>),
    Paragraph(Vec<ReleaseInline>),
    UnorderedList(Vec<Vec<ReleaseInline>>),
}

#[derive(Clone, Copy)]
enum InlineMarker {
    Strong,
    Code,
    Link,
}

fn safe_release_link(href: &str) -> bool {
    href.starts_with("https://") || href.starts_with("http://")
}

fn parse_release_link(text: &str) -> Option<(&str, &str, usize)> {
    let label_end = text.find("](")?;
    let href_start = label_end + 2;
    let href_end = href_start + text[href_start..].find(')')?;
    let href = &text[href_start..href_end];
    safe_release_link(href).then_some((&text[1..label_end], href, href_end + 1))
}

fn parse_release_inlines(text: &str) -> Vec<ReleaseInline> {
    let mut inlines = Vec::new();
    let mut remaining = text;

    while !remaining.is_empty() {
        let next_marker = [
            remaining
                .find("**")
                .map(|offset| (offset, InlineMarker::Strong)),
            remaining
                .find('`')
                .map(|offset| (offset, InlineMarker::Code)),
            remaining
                .find('[')
                .map(|offset| (offset, InlineMarker::Link)),
        ]
        .into_iter()
        .flatten()
        .min_by_key(|(offset, _)| *offset);

        let Some((offset, marker)) = next_marker else {
            inlines.push(ReleaseInline::Text(remaining.to_string()));
            break;
        };

        if offset > 0 {
            inlines.push(ReleaseInline::Text(remaining[..offset].to_string()));
            remaining = &remaining[offset..];
            continue;
        }

        match marker {
            InlineMarker::Strong => {
                if let Some(end) = remaining[2..].find("**") {
                    let end = end + 2;
                    inlines.push(ReleaseInline::Strong(remaining[2..end].to_string()));
                    remaining = &remaining[end + 2..];
                } else {
                    inlines.push(ReleaseInline::Text("**".to_string()));
                    remaining = &remaining[2..];
                }
            }
            InlineMarker::Code => {
                if let Some(end) = remaining[1..].find('`') {
                    let end = end + 1;
                    inlines.push(ReleaseInline::Code(remaining[1..end].to_string()));
                    remaining = &remaining[end + 1..];
                } else {
                    inlines.push(ReleaseInline::Text("`".to_string()));
                    remaining = &remaining[1..];
                }
            }
            InlineMarker::Link => {
                if let Some((label, href, consumed)) = parse_release_link(remaining) {
                    inlines.push(ReleaseInline::Link {
                        label: label.to_string(),
                        href: href.to_string(),
                    });
                    remaining = &remaining[consumed..];
                } else {
                    inlines.push(ReleaseInline::Text("[".to_string()));
                    remaining = &remaining[1..];
                }
            }
        }
    }

    inlines
}

fn flush_release_paragraph(blocks: &mut Vec<ReleaseBlock>, lines: &mut Vec<String>) {
    if !lines.is_empty() {
        blocks.push(ReleaseBlock::Paragraph(parse_release_inlines(
            &std::mem::take(lines).join(" "),
        )));
    }
}

fn flush_release_list(blocks: &mut Vec<ReleaseBlock>, items: &mut Vec<String>) {
    if !items.is_empty() {
        blocks.push(ReleaseBlock::UnorderedList(
            std::mem::take(items)
                .into_iter()
                .map(|item| parse_release_inlines(&item))
                .collect(),
        ));
    }
}

fn parse_release_markdown(body: &str) -> Vec<ReleaseBlock> {
    let mut blocks = Vec::new();
    let mut paragraph = Vec::new();
    let mut list = Vec::new();

    for line in body.lines().map(str::trim) {
        if line == "## What's Changed" || line == "## What’s Changed" {
            flush_release_paragraph(&mut blocks, &mut paragraph);
            flush_release_list(&mut blocks, &mut list);
            break;
        }
        if line.is_empty() {
            flush_release_paragraph(&mut blocks, &mut paragraph);
            flush_release_list(&mut blocks, &mut list);
        } else if let Some(text) = line.strip_prefix("#### ") {
            flush_release_paragraph(&mut blocks, &mut paragraph);
            flush_release_list(&mut blocks, &mut list);
            blocks.push(ReleaseBlock::Heading4(parse_release_inlines(text)));
        } else if let Some(text) = line.strip_prefix("### ") {
            flush_release_paragraph(&mut blocks, &mut paragraph);
            flush_release_list(&mut blocks, &mut list);
            blocks.push(ReleaseBlock::Heading3(parse_release_inlines(text)));
        } else if let Some(text) = line.strip_prefix("## ").or_else(|| line.strip_prefix("# ")) {
            flush_release_paragraph(&mut blocks, &mut paragraph);
            flush_release_list(&mut blocks, &mut list);
            blocks.push(ReleaseBlock::Heading2(parse_release_inlines(text)));
        } else if let Some(text) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
            flush_release_paragraph(&mut blocks, &mut paragraph);
            list.push(text.to_string());
        } else if let Some(text) = line
            .strip_prefix("**")
            .and_then(|text| text.strip_suffix("**"))
            .filter(|text| !text.contains("**"))
        {
            flush_release_paragraph(&mut blocks, &mut paragraph);
            flush_release_list(&mut blocks, &mut list);
            blocks.push(ReleaseBlock::Heading4(parse_release_inlines(text)));
        } else {
            flush_release_list(&mut blocks, &mut list);
            paragraph.push(line.to_string());
        }
    }

    flush_release_paragraph(&mut blocks, &mut paragraph);
    flush_release_list(&mut blocks, &mut list);
    blocks
}

fn release_inlines_view(inlines: Vec<ReleaseInline>) -> impl IntoView {
    inlines
        .into_iter()
        .map(|inline| match inline {
            ReleaseInline::Text(text) => view! { <span>{text}</span> }.into_any(),
            ReleaseInline::Strong(text) => view! { <strong>{text}</strong> }.into_any(),
            ReleaseInline::Code(text) => view! { <code>{text}</code> }.into_any(),
            ReleaseInline::Link { label, href } => view! {
                <a href=href target="_blank" rel="noopener noreferrer">{label}</a>
            }
            .into_any(),
        })
        .collect_view()
}

#[component]
pub(crate) fn WhatsNew() -> impl IntoView {
    let latest_release = use_latest_release();

    view! {
        <section class="sec whats-new" id="whats-new" data-title="What's new">
            <div class="wrap">
                <h2>{move_tr!("new-title")}</h2>
                <Suspense fallback=|| view! { <p class="prose release-loading">"Loading the latest release."</p> }>
                    {move || latest_release.get().map(|release| {
                        let date = release.published_at.get(..10).unwrap_or("").to_string();
                        let version = if release.tag_name.is_empty() {
                            "Latest".to_string()
                        } else {
                            release.tag_name.clone()
                        };
                        let blocks = parse_release_markdown(&release.body);
                        let name = release.name.trim();
                        let name = (!name.is_empty()
                            && name.trim_start_matches('v') != version.trim_start_matches('v'))
                        .then(|| name.to_string());
                        view! {
                            <details class="release-notes">
                                <summary class="release-summary">
                                    <span class="release-heads">
                                        <span class="release-version">{version}</span>
                                        <span class="release-date">{date}</span>
                                        {name.map(|name| view! { <strong class="release-name">{name}</strong> })}
                                    </span>
                                    <Icon name="chevron-down" class="release-chevron"/>
                                </summary>
                                <div class="release-body">
                                    {blocks.into_iter().map(|block| match block {
                                        ReleaseBlock::Heading2(inlines) => view! { <h3>{release_inlines_view(inlines)}</h3> }.into_any(),
                                        ReleaseBlock::Heading3(inlines) => view! { <h4>{release_inlines_view(inlines)}</h4> }.into_any(),
                                        ReleaseBlock::Heading4(inlines) => view! { <h4>{release_inlines_view(inlines)}</h4> }.into_any(),
                                        ReleaseBlock::Paragraph(inlines) => view! { <p>{release_inlines_view(inlines)}</p> }.into_any(),
                                        ReleaseBlock::UnorderedList(items) => view! {
                                            <ul>
                                                {items.into_iter().map(|item| view! {
                                                    <li>{release_inlines_view(item)}</li>
                                                }).collect_view()}
                                            </ul>
                                        }.into_any(),
                                    }).collect_view()}
                                </div>
                            </details>
                        }
                    })}
                </Suspense>
            </div>
        </section>
    }
}

#[cfg(test)]
mod release_markdown_tests {
    use super::*;

    #[test]
    fn groups_paragraph_lines_and_list_items() {
        let blocks = parse_release_markdown(
            "First line\ncontinues here.\n\n## Highlights\n\n- One\n- Two\n\n## What's Changed\n- Hidden",
        );

        assert!(matches!(&blocks[0], ReleaseBlock::Paragraph(_)));
        assert!(matches!(&blocks[1], ReleaseBlock::Heading2(_)));
        assert!(matches!(&blocks[2], ReleaseBlock::UnorderedList(items) if items.len() == 2));
        assert_eq!(blocks.len(), 3);
    }

    #[test]
    fn preserves_safe_inline_markdown() {
        assert_eq!(
            parse_release_inlines("**Fast.** Uses `cache` and [notes](https://example.com)."),
            vec![
                ReleaseInline::Strong("Fast.".into()),
                ReleaseInline::Text(" Uses ".into()),
                ReleaseInline::Code("cache".into()),
                ReleaseInline::Text(" and ".into()),
                ReleaseInline::Link {
                    label: "notes".into(),
                    href: "https://example.com".into(),
                },
                ReleaseInline::Text(".".into()),
            ]
        );
    }
}
