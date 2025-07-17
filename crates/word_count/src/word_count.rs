use editor::Editor;
use gpui::ParentElement;
use gpui::{App, Entity, Subscription, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use std::{fmt::Write, time::Duration};
use ui::{Button, Context, FluentBuilder, IntoElement, LabelSize, Render, Window, div};
use workspace::{StatusItemView, item::ItemHandle};

#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub(crate) struct SelectionStats {
    pub words: usize,
}

pub struct WordCount {
    selected_count: Option<SelectionStats>,
    update_word_count: Task<()>,
    _observe_active_editor: Option<Subscription>,
}

/// Count words in a text string using whitespace as delimiter
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

impl WordCount {
    pub fn new() -> Self {
        Self {
            selected_count: None,
            update_word_count: Task::ready(()),
            _observe_active_editor: None,
        }
    }

    fn update_word_count(
        &mut self,
        editor: Entity<Editor>,
        debounce: Option<Duration>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let editor = editor.downgrade();
        self.update_word_count = cx.spawn_in(window, async move |word_count, cx| {
            let is_singleton = editor
                .update(cx, |editor, cx| editor.buffer().read(cx).is_singleton())
                .ok()
                .unwrap_or(true);

            if !is_singleton {
                if let Some(debounce) = debounce {
                    cx.background_executor().timer(debounce).await;
                }
            }

            editor
                .update(cx, |editor, cx| {
                    word_count.update(cx, |word_count, cx| {
                        let snapshot = editor.buffer().read(cx).snapshot(cx);
                        let full_text = snapshot.text();
                        let words = count_words(&full_text);
                        word_count.selected_count = Some(SelectionStats { words });
                        cx.notify();
                    })
                })
                .ok()
                .transpose()
                .ok()
                .flatten();
        });
    }

    fn write_position(&self, text: &mut String, stats: &SelectionStats) {
        write!(
            text,
            "{} word{}",
            stats.words,
            if stats.words != 1 { "s" } else { "" }
        )
        .unwrap();
    }
}

impl Render for WordCount {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().when_some(self.selected_count, |el, stats| {
            let mut text = String::new();
            self.write_position(&mut text, &stats);
            el.child(Button::new("word-count", text).label_size(LabelSize::Small))
        })
    }
}

const UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);

impl StatusItemView for WordCount {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.act_as::<Editor>(cx)) {
            self._observe_active_editor = Some(cx.observe_in(
                &editor,
                window,
                |word_count, editor, window, cx| {
                    Self::update_word_count(word_count, editor, Some(UPDATE_DEBOUNCE), window, cx)
                },
            ));
            self.update_word_count(editor, None, window, cx);
        } else {
            self.selected_count = None;
            self._observe_active_editor = None;
        }

        cx.notify();
    }
}

#[derive(Clone, Copy, Default, PartialEq, JsonSchema, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LineIndicatorFormat {
    Short,
    #[default]
    Long,
}

#[derive(Clone, Copy, Default, JsonSchema, Deserialize, Serialize)]
#[serde(transparent)]
pub(crate) struct LineIndicatorFormatContent(LineIndicatorFormat);

impl Settings for LineIndicatorFormat {
    const KEY: Option<&'static str> = Some("line_indicator_format");

    type FileContent = Option<LineIndicatorFormatContent>;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> anyhow::Result<Self> {
        let format = [sources.release_channel, sources.user]
            .into_iter()
            .find_map(|value| value.copied().flatten())
            .unwrap_or(sources.default.ok_or_else(Self::missing_default)?);

        Ok(format.0)
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}
