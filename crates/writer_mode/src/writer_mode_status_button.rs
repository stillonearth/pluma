// use editor::EditorSettings;
// use settings::Settings as _;
use ui::{
    ButtonCommon, ButtonLike, Clickable, Color, Context, Icon, IconName, IconSize, ParentElement,
    Render, Styled, Tooltip, Window, h_flex,
};
use workspace::{ItemHandle, StatusItemView};

pub struct WriterModeButton;

impl WriterModeButton {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for WriterModeButton {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let button = h_flex().gap_2();

        button.child(
            ButtonLike::new("writer-mode-indicator")
                .child(
                    Icon::new(IconName::Pencil)
                        .size(IconSize::Small)
                        .color(Color::Default),
                )
                .tooltip(|window, cx| {
                    Tooltip::for_action(
                        "Writer Mode",
                        &workspace::WriterMode::default(),
                        window,
                        cx,
                    )
                })
                .on_click(cx.listener(|_this, _, window, cx| {
                    window.dispatch_action(Box::new(workspace::WriterMode::default()), cx);
                })),
        )
    }
}

impl StatusItemView for WriterModeButton {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}
