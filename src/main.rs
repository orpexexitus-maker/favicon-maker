use native_windows_gui as nwg;
use native_windows_gui::NativeUi;
use std::cell::RefCell;
use std::path::PathBuf;

const ICON_SIZES: [u32; 6] = [16, 32, 48, 64, 128, 256];

#[derive(Default)]
pub struct FaviconMaker {
    pub window: nwg::Window,
    pub src_label: nwg::Label,
    pub src_box: nwg::TextInput,
    pub src_btn: nwg::Button,
    pub out_label: nwg::Label,
    pub out_box: nwg::TextInput,
    pub out_btn: nwg::Button,
    pub size_label: nwg::Label,
    pub checks: RefCell<Vec<nwg::CheckBox>>,
    pub gen_btn: nwg::Button,
    pub status: nwg::Label,
    pub src_path: RefCell<Option<PathBuf>>,
}

fn build_ico(
    src: &PathBuf,
    out: &PathBuf,
    sizes: &[u32],
) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(src)?.to_rgba8();

    let mut pngs: Vec<Vec<u8>> = Vec::with_capacity(sizes.len());
    for &s in sizes {
        let resized = image::imageops::resize(&img, s, s, image::imageops::FilterType::Lanczos3);
        let mut buf = Vec::new();
        let enc = image::codecs::png::PngEncoder::new(&mut buf);
        image::ImageEncoder::write_image(
            enc,
            resized.as_raw(),
            s,
            s,
            image::ExtendedColorType::Rgba8,
        )?;
        pngs.push(buf);
    }

    let n = sizes.len() as u16;
    let header_size = 6 + 16 * sizes.len();
    let mut offsets: Vec<u32> = Vec::with_capacity(sizes.len());
    let mut cursor = header_size as u32;
    for p in &pngs {
        offsets.push(cursor);
        cursor += p.len() as u32;
    }

    let mut out_bytes = Vec::with_capacity(cursor as usize);
    out_bytes.extend_from_slice(&0u16.to_le_bytes());
    out_bytes.extend_from_slice(&1u16.to_le_bytes());
    out_bytes.extend_from_slice(&n.to_le_bytes());

    for (i, &s) in sizes.iter().enumerate() {
        let dim = if s == 256 { 0u8 } else { s as u8 };
        out_bytes.push(dim);
        out_bytes.push(dim);
        out_bytes.push(0);
        out_bytes.push(0);
        out_bytes.extend_from_slice(&1u16.to_le_bytes());
        out_bytes.extend_from_slice(&32u16.to_le_bytes());
        out_bytes.extend_from_slice(&(pngs[i].len() as u32).to_le_bytes());
        out_bytes.extend_from_slice(&offsets[i].to_le_bytes());
    }
    for p in &pngs {
        out_bytes.extend_from_slice(p);
    }

    std::fs::write(out, out_bytes)?;
    Ok(())
}

fn generate(ui: &FaviconMaker) {
    let src = match ui.src_path.borrow().clone() {
        Some(p) => p,
        None => {
            nwg::modal_error_message(&ui.window, "Error", "Pick a source image first.");
            return;
        }
    };

    let out_str = ui.out_box.text();
    if out_str.trim().is_empty() {
        nwg::modal_error_message(&ui.window, "Error", "Choose an output .ico path.");
        return;
    }
    let out = PathBuf::from(out_str);

    let sizes: Vec<u32> = ICON_SIZES
        .iter()
        .zip(ui.checks.borrow().iter())
        .filter(|(_, c)| c.check_state() == nwg::CheckBoxState::Checked)
        .map(|(s, _)| *s)
        .collect();

    if sizes.is_empty() {
        nwg::modal_error_message(&ui.window, "Error", "Select at least one size.");
        return;
    }

    match build_ico(&src, &out, &sizes) {
        Ok(()) => {
            ui.status.set_text(&format!("Done -> {}", out.display()));
            nwg::modal_info_message(
                &ui.window,
                "Success",
                &format!("Favicon saved to:\n{}", out.display()),
            );
        }
        Err(e) => {
            ui.status.set_text("Failed.");
            nwg::modal_error_message(&ui.window, "Error", &e.to_string());
        }
    }
}

mod ui {
    use super::*;
    use native_windows_gui as nwg;

    #[derive(Default)]
    pub struct FaviconMakerUi {
        pub inner: FaviconMaker,
    }

    impl NativeUi<FaviconMakerUi> for FaviconMakerUi {
        fn build_ui(mut ui: FaviconMakerUi) -> Result<FaviconMakerUi, nwg::NwgError> {
            // Window is built directly on ui.inner.window via the builder.
            nwg::Window::builder()
                .size((560, 340))
                .position((300, 200))
                .title("Favicon Maker")
                .build(&mut ui.inner.window)?;

            nwg::Label::builder()
                .text("Source image:")
                .parent(&ui.inner.window)
                .position((15, 15))
                .size((200, 20))
                .build(&mut ui.inner.src_label)?;

            nwg::TextInput::builder()
                .parent(&ui.inner.window)
                .position((15, 38))
                .size((430, 24))
                .readonly(true)
                .build(&mut ui.inner.src_box)?;

            nwg::Button::builder()
                .text("Browse...")
                .parent(&ui.inner.window)
                .position((455, 38))
                .size((90, 24))
                .build(&mut ui.inner.src_btn)?;

            nwg::Label::builder()
                .text("Output .ico:")
                .parent(&ui.inner.window)
                .position((15, 75))
                .size((200, 20))
                .build(&mut ui.inner.out_label)?;

            nwg::TextInput::builder()
                .parent(&ui.inner.window)
                .position((15, 98))
                .size((430, 24))
                .build(&mut ui.inner.out_box)?;

            nwg::Button::builder()
                .text("Save as...")
                .parent(&ui.inner.window)
                .position((455, 98))
                .size((90, 24))
                .build(&mut ui.inner.out_btn)?;

            nwg::Label::builder()
                .text("Sizes to include:")
                .parent(&ui.inner.window)
                .position((15, 140))
                .size((200, 20))
                .build(&mut ui.inner.size_label)?;

            for (i, s) in ICON_SIZES.iter().enumerate() {
                let mut cb = nwg::CheckBox::default();
                let default_checked = *s != 64;
                nwg::CheckBox::builder()
                    .text(&format!("{}x{}", s, s))
                    .parent(&ui.inner.window)
                    .position((15 + (i as i32 % 3) * 120, 165 + (i as i32 / 3) * 30))
                    .size((110, 24))
                    .check_state(if default_checked {
                        nwg::CheckBoxState::Checked
                    } else {
                        nwg::CheckBoxState::Unchecked
                    })
                    .build(&mut cb)?;
                ui.inner.checks.borrow_mut().push(cb);
            }

            nwg::Label::builder()
                .text("Ready.")
                .parent(&ui.inner.window)
                .position((15, 235))
                .size((530, 20))
                .build(&mut ui.inner.status)?;

            nwg::Button::builder()
                .text("Generate Favicon")
                .parent(&ui.inner.window)
                .position((15, 265))
                .size((530, 40))
                .build(&mut ui.inner.gen_btn)?;

            Ok(ui)
        }
    }

    pub fn handle_events(
        ui: &FaviconMakerUi,
        evt: nwg::Event,
        _evt_data: nwg::EventData,
        handle: nwg::ControlHandle,
    ) {
        if evt != nwg::Event::OnButtonClick {
            return;
        }

        if handle == ui.inner.src_btn.handle {
            let mut dialog = nwg::FileDialog::default();
            if nwg::FileDialog::builder()
                .title("Select image")
                .action(nwg::FileDialogAction::Open)
                .filters("Images(*.png;*.jpg;*.jpeg;*.bmp;*.gif;*.webp)|All(*.*)")
                .build(&mut dialog)
                .is_ok()
            {
                if dialog.run(Some(&ui.inner.window)) {
                    if let Ok(path) = dialog.get_selected_item() {
                        let path_str = path.to_string_lossy().to_string();
                        let pb = PathBuf::from(&path_str);
                        ui.inner.src_box.set_text(&path_str);
                        if ui.inner.out_box.text().trim().is_empty() {
                            let ico = pb.with_extension("ico");
                            ui.inner.out_box.set_text(&ico.to_string_lossy());
                        }
                        *ui.inner.src_path.borrow_mut() = Some(pb);
                    }
                }
            }
        } else if handle == ui.inner.out_btn.handle {
            let mut dialog = nwg::FileDialog::default();
            if nwg::FileDialog::builder()
                .title("Save favicon as")
                .action(nwg::FileDialogAction::Save)
                .filters("Icon(*.ico)")
                .build(&mut dialog)
                .is_ok()
            {
                if dialog.run(Some(&ui.inner.window)) {
                    if let Ok(path) = dialog.get_selected_item() {
                        ui.inner.out_box.set_text(&path.to_string_lossy());
                    }
                }
            }
        } else if handle == ui.inner.gen_btn.handle {
            generate(&ui.inner);
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init NWG");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set font");

    let ui = ui::FaviconMakerUi::build_ui(Default::default()).expect("Failed to build UI");

    let ui_handle = ui.inner.window.handle;
    let mut handler = nwg::full_bind_event_handler(&ui_handle, move |evt, evt_data, handle| {
        ui::handle_events(&ui, evt, evt_data, handle);
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&mut handler);
}