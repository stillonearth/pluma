// use collab_ui::collab_panel;
use gpui::{Menu, MenuItem, OsAction};
use settings_ui::keybindings;

pub fn app_menus() -> Vec<Menu> {
    use zed_actions::Quit;

    vec![
        Menu {
            name: "Zed".into(),
            items: vec![
                MenuItem::action("About Pluma…", zed_actions::About),
                MenuItem::action("Check for Updates", auto_update::Check),
                MenuItem::separator(),
                MenuItem::submenu(Menu {
                    name: "Settings".into(),
                    items: vec![
                        MenuItem::action("Open Settings", super::OpenSettings),
                        MenuItem::action("Open Key Bindings", keybindings::OpenKeymapEditor),
                        MenuItem::action("Open Default Settings", super::OpenDefaultSettings),
                        MenuItem::action(
                            "Open Default Key Bindings",
                            zed_actions::OpenDefaultKeymap,
                        ),
                        MenuItem::action("Open Project Settings", super::OpenProjectSettings),
                        MenuItem::action(
                            "Select Theme...",
                            zed_actions::theme_selector::Toggle::default(),
                        ),
                    ],
                }),
                MenuItem::separator(),
                MenuItem::submenu(Menu {
                    name: "Services".into(),
                    items: vec![],
                }),
                MenuItem::separator(),
                // MenuItem::action("Extensions", zed_actions::Extensions::default()),
                // MenuItem::action("Install CLI", install_cli::Install),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::action("Hide Zed", super::Hide),
                #[cfg(target_os = "macos")]
                MenuItem::action("Hide Others", super::HideOthers),
                #[cfg(target_os = "macos")]
                MenuItem::action("Show All", super::ShowAll),
                MenuItem::separator(),
                MenuItem::action("Quit Pluma", Quit),
            ],
        },
        Menu {
            name: "File".into(),
            items: vec![
                MenuItem::action("New", workspace::NewFile),
                MenuItem::action("New Window", workspace::NewWindow),
                MenuItem::separator(),
                #[cfg(not(target_os = "macos"))]
                MenuItem::action("Open File...", workspace::OpenFiles),
                MenuItem::action(
                    if cfg!(not(target_os = "macos")) {
                        "Open Folder..."
                    } else {
                        "Open…"
                    },
                    workspace::Open,
                ),
                MenuItem::action(
                    "Open Recent...",
                    zed_actions::OpenRecent {
                        create_new_window: false,
                    },
                ),
                MenuItem::action(
                    "Open Remote...",
                    zed_actions::OpenRemote {
                        create_new_window: false,
                        from_existing_connection: false,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action("Add Folder to Project…", workspace::AddFolderToProject),
                MenuItem::separator(),
                MenuItem::action("Save", workspace::Save { save_intent: None }),
                MenuItem::action("Save As…", workspace::SaveAs),
                MenuItem::action("Save All", workspace::SaveAll { save_intent: None }),
                MenuItem::separator(),
                MenuItem::action(
                    "Close Editor",
                    workspace::CloseActiveItem {
                        save_intent: None,
                        close_pinned: true,
                    },
                ),
                MenuItem::action("Close Window", workspace::CloseWindow),
            ],
        },
        Menu {
            name: "Edit".into(),
            items: vec![
                MenuItem::os_action("Undo", editor::actions::Undo, OsAction::Undo),
                MenuItem::os_action("Redo", editor::actions::Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action("Cut", editor::actions::Cut, OsAction::Cut),
                MenuItem::os_action("Copy", editor::actions::Copy, OsAction::Copy),
                MenuItem::action("Copy and Trim", editor::actions::CopyAndTrim),
                MenuItem::os_action("Paste", editor::actions::Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::action("Find", search::buffer_search::Deploy::find()),
                MenuItem::action("Find In Project", workspace::DeploySearch::find()),
                MenuItem::separator(),
                MenuItem::action(
                    "Toggle Line Comment",
                    editor::actions::ToggleComments::default(),
                ),
            ],
        },
        Menu {
            name: "Selection".into(),
            items: vec![
                MenuItem::os_action(
                    "Select All",
                    editor::actions::SelectAll,
                    OsAction::SelectAll,
                ),
                MenuItem::action("Expand Selection", editor::actions::SelectLargerSyntaxNode),
                MenuItem::action("Shrink Selection", editor::actions::SelectSmallerSyntaxNode),
                MenuItem::separator(),
                MenuItem::action("Add Cursor Above", editor::actions::AddSelectionAbove),
                MenuItem::action("Add Cursor Below", editor::actions::AddSelectionBelow),
                MenuItem::action(
                    "Select Next Occurrence",
                    editor::actions::SelectNext {
                        replace_newest: false,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action("Move Line Up", editor::actions::MoveLineUp),
                MenuItem::action("Move Line Down", editor::actions::MoveLineDown),
                MenuItem::action("Duplicate Selection", editor::actions::DuplicateLineDown),
            ],
        },
        Menu {
            name: "View".into(),
            items: vec![
                MenuItem::action(
                    "Zoom In",
                    zed_actions::IncreaseBufferFontSize { persist: true },
                ),
                MenuItem::action(
                    "Zoom Out",
                    zed_actions::DecreaseBufferFontSize { persist: true },
                ),
                MenuItem::action(
                    "Reset Zoom",
                    zed_actions::ResetBufferFontSize { persist: true },
                ),
                MenuItem::separator(),
                MenuItem::action("Toggle Left Dock", workspace::ToggleLeftDock),
                MenuItem::action("Toggle Right Dock", workspace::ToggleRightDock),
                MenuItem::action("Toggle Bottom Dock", workspace::ToggleBottomDock),
                MenuItem::action("Close All Docks", workspace::CloseAllDocks),
                MenuItem::submenu(Menu {
                    name: "Editor Layout".into(),
                    items: vec![
                        MenuItem::action("Split Up", workspace::SplitUp),
                        MenuItem::action("Split Down", workspace::SplitDown),
                        MenuItem::action("Split Left", workspace::SplitLeft),
                        MenuItem::action("Split Right", workspace::SplitRight),
                    ],
                }),
                MenuItem::separator(),
                MenuItem::action("Project Panel", project_panel::ToggleFocus),
            ],
        },
        Menu {
            name: "Window".into(),
            items: vec![
                MenuItem::action("Minimize", super::Minimize),
                MenuItem::action("Zoom", super::Zoom),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: "Help".into(),
            items: vec![
                MenuItem::action(
                    "View Release Notes",
                    auto_update_ui::ViewReleaseNotesLocally,
                ),
                MenuItem::action("View Dependency Licenses", zed_actions::OpenLicenses),
                MenuItem::action("Show Welcome", workspace::Welcome),
                MenuItem::separator(),
                MenuItem::action(
                    "Documentation",
                    super::OpenBrowser {
                        url: "https://elpluma.dev/docs".into(),
                    },
                ),
                MenuItem::action(
                    "Pluma Twitter",
                    super::OpenBrowser {
                        url: "https://twitter.com/zeddotdev".into(),
                    },
                ),
            ],
        },
    ]
}
