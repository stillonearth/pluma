// use editor::EditorSettings;
use gpui::App;
use workspace::{Workspace, WriterMode};

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(move |workspace, _action: &WriterMode, window, cx| {
            if workspace.has_active_modal(window, cx) {
                cx.propagate();
                return;
            }

            window.dispatch_action(Box::new(editor::actions::ToggleLineNumbers::default()), cx);

            workspace.close_all_docks(window, cx);
            for pane in workspace.panes() {
                pane.update(cx, |pane, cx| {
                    let should_display_tab_bar = pane.get_should_display_tab_bar();
                    let display_tab_bar = should_display_tab_bar(window, cx);
                    pane.set_should_display_tab_bar(move |_, _| !display_tab_bar);

                    let should_display_toolbar = pane.get_should_display_toolbar();
                    let display_toolbar = should_display_toolbar(window, cx);
                    pane.set_should_display_toolbar(move |_, _| !display_toolbar);
                });
            }

            cx.notify();
        });
    })
    .detach();
}
