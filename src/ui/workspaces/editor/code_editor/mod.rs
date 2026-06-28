use crate::ui::workspaces::EditorUi;
use eframe::egui;
use egui_extras::syntax_highlighting::{CodeTheme, SyntectSettings, highlight_with};
use syntect::highlighting::ThemeSet;

impl EditorUi {
    pub(super) fn code_editor(&mut self, ui: &mut egui::Ui) {
        let code = r#"
import std
import math/float
import convert
import audio

input notes = [audio.Voice(); audio.max_voices]
output sample = audio.zero_sample()

let pi2 = 6.28318530
let vib_rate = 6.0
let vib_depth = 0.003
let fm_ratio = 3.0
let fm_depth = 0.3

func main() {
    var out = 0.0

    var i = 0
    loop audio.max_voices {
        if notes[i].is_active {
            let t = notes[i].age
            let base_freq = 440.0 * float.pow(2.0, (notes[i].pitch - 69.0) / 12.0)

            if base_freq > 500.0 {
                let vib = float.fast_sin(pi2 * vib_rate * t) * vib_depth
                let vib_freq = base_freq * (1.0 + vib)
                let mod_sig = float.fast_sin(vib_freq * fm_ratio * t * pi2) * fm_depth
                let carrier = float.sgn(float.fast_sin(vib_freq * t * pi2 + mod_sig))
                out = out + carrier * notes[i].velocity * 0.3
            } else {
                let mod_sig = float.fast_sin(base_freq * fm_ratio * t * pi2) * fm_depth
                let carrier = float.sgn(float.sin(base_freq * t * pi2 + mod_sig))
                out = out + carrier * notes[i].velocity * 0.3
            }
        }
        i = i + 1
    }

    let fm_out = (out / convert.int_to_float(audio.max_voices))

    sample[0] = fm_out
    sample[1] = fm_out
}"#;
        let theme = CodeTheme::from_memory(ui.ctx(), ui.style());

        let syntect_settings = SyntectSettings {
            ps: self.syntax_set.clone(),
            ts: ThemeSet::load_defaults(),
        };

        let highlighted_str =
            highlight_with(ui.ctx(), ui.style(), &theme, code, "rs", &syntect_settings);
        ui.add(egui::Label::new(highlighted_str).selectable(true));
    }
}
