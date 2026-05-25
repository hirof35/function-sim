use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

// 多項式の計算ロジック（ホーナー法）
struct Polynomial {
    coefficients: Vec<f64>,
}

impl Polynomial {
    fn new(coefficients: Vec<f64>) -> Self {
        Self { coefficients }
    }

    fn evaluate(&self, x: f64) -> f64 {
        self.coefficients
            .iter()
            .rev()
            .fold(0.0, |acc, &coeff| acc * x + coeff)
    }
}

// GUIアプリケーションの状態管理
struct SimulatorApp {
    // 各次数の係数（テキストボックス入力用に文字列で保持）
    coeff_0_str: String, // 定数項 (c)
    coeff_1_str: String, // 1次の係数 (b)
    coeff_2_str: String, // 2次の係数 (a)
    coeff_3_str: String, // 3次の係数 (d)
    
    // X軸の範囲（直接入力用）
    x_min_str: String,
    x_max_str: String,
}

impl Default for SimulatorApp {
    fn default() -> Self {
        Self {
            // 初期値: f(x) = 1x² - 4x + 4
            coeff_0_str: "4.0".to_string(),
            coeff_1_str: "-4.0".to_string(),
            coeff_2_str: "1.0".to_string(),
            coeff_3_str: "0.0".to_string(),
            
            x_min_str: "-5.0".to_string(),
            x_max_str: "5.0".to_string(),
        }
    }
}

impl eframe::App for SimulatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. 入力された文字列を数値に変換（パース失敗時は 0.0 や初期値を設定）
        let c = self.coeff_0_str.parse::<f64>().unwrap_or(0.0);
        let b = self.coeff_1_str.parse::<f64>().unwrap_or(0.0);
        let a = self.coeff_2_str.parse::<f64>().unwrap_or(0.0);
        let d = self.coeff_3_str.parse::<f64>().unwrap_or(0.0);

        let x_min = self.x_min_str.parse::<f64>().unwrap_or(-5.0);
        let x_max = self.x_max_str.parse::<f64>().unwrap_or(5.0);

        // 多項式オブジェクトの生成: f(x) = d*x³ + a*x² + b*x + c
        let poly = Polynomial::new(vec![c, b, a, d]);

        // 2. 左側の設定パネル（幅を250〜300の間に制限）
        egui::SidePanel::left("control_panel").width_range(250.0..=300.0).show(ctx, |ui| {
            ui.heading("📊 関数計算シミュレータ");
            ui.separator();

            // 現在の数式プレビュー
            ui.label("【 現在の数式 】");
            ui.colored_label(
                egui::Color32::from_rgb(100, 200, 255),
                format!("f(x) = ({})x³ + ({})x² + ({})x + ({})", d, a, b, c)
            );
            ui.add_space(10.0);

            // 係数の入力フォーム
            ui.label("【 係数の入力 】");
            ui.horizontal(|ui| {
                ui.label("3次の係数 (x³):");
                ui.text_edit_singleline(&mut self.coeff_3_str);
            });
            ui.horizontal(|ui| {
                ui.label("2次の係数 (x²):");
                ui.text_edit_singleline(&mut self.coeff_2_str);
            });
            ui.horizontal(|ui| {
                ui.label("1次の係数 (x) :");
                ui.text_edit_singleline(&mut self.coeff_1_str);
            });
            ui.horizontal(|ui| {
                ui.label("定数項   (c) :");
                ui.text_edit_singleline(&mut self.coeff_0_str);
            });

            ui.add_space(20.0);

            // X軸の範囲入力
            ui.label("【 X軸の描画範囲 】");
            ui.horizontal(|ui| {
                ui.label("最小値 (x 最小):");
                ui.text_edit_singleline(&mut self.x_min_str);
            });
            ui.horizontal(|ui| {
                ui.label("最大値 (x 最大):");
                ui.text_edit_singleline(&mut self.x_max_str);
            });
            
            ui.add_space(25.0);
            
            // リセットボタン
            if ui.button("🔄 数値を初期化").clicked() {
                *self = SimulatorApp::default();
            }
        });

        // 3. 右側のグラフ描画エリア
        egui::CentralPanel::default().show(ctx, |ui| {
            // エラーチェック（最小値と最大値が逆転していないか）
            if x_min < x_max {
                // 描画範囲の広さに応じて、点の密度（刻み幅）を自動調整
                let step = (x_max - x_min) / 200.0; 
                let mut points = Vec::new();
                let mut x = x_min;
                
                while x <= x_max + (step / 2.0) {
                    let y = poly.evaluate(x);
                    points.push([x, y]);
                    x += step;
                }

                // グラフの線の設定
                let line = Line::new(PlotPoints::from(points))
                    .name("f(x)")
                    .width(2.5); // 線を少し太くして見やすく
                
                // プロット領域の描画
                Plot::new("function_plot")
                    .view_aspect(1.0)
                    .data_aspect(1.0) // 1マスの縦横比を1:1に固定
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
                    });
            } else {
                // 入力エラー時の警告表示
                ui.centered_and_justified(|ui| {
                    ui.colored_label(
                        egui::Color32::LIGHT_RED,
                        "⚠ エラー: 最小値には、最大値より小さい数値を入力してください。",
                    );
                });
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    // ウィンドウの初期設定
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([850.0, 600.0])
            .with_title("自由入力式・関数計算シミュレータ"),
        ..Default::default()
    };
    
    // アプリケーションの起動
    eframe::run_native(
        "自由入力式・関数計算シミュレータ",
        options,
        Box::new(|cc| {
            // -------------------------------------------------------------
            // 【重要】日本語フォント（OS標準フォント）の読み込み設定
            // -------------------------------------------------------------
            let mut fonts = egui::FontDefinitions::default();

            // Windowsの「MSゴシック」やMacの標準フォントなど、環境に合わせてパスを試す
            let font_paths = [
                "C:\\Windows\\Fonts\\msgothic.ttc",    // Windows用 (MSゴシック)
                "C:\\Windows\\Fonts\\meiryo.ttc",      // Windows用 (メイリオ)
                "/System/Library/Fonts/NotoSansCJK-Regular.ttc", // Mac用
                "/usr/share/fonts/truetype/fonts-japanese-gothic.ttf", // Linux用
            ];

            let mut font_loaded = false;
            for path in font_paths {
                if let Ok(font_data) = std::fs::read(path) {
                    // フォントデータを登録
                    fonts.font_data.insert(
                        "my_japanese_font".to_owned(),
                        egui::FontData::from_owned(font_data),
                    );
                    
                    // デフォルトのフォント（Proportional と Monospace）に設定
                    fonts.families.get_mut(&egui::FontFamily::Proportional)
                        .unwrap()
                        .insert(0, "my_japanese_font".to_owned());
                    fonts.families.get_mut(&egui::FontFamily::Monospace)
                        .unwrap()
                        .insert(0, "my_japanese_font".to_owned());
                    
                    font_loaded = true;
                    break; // 1つ見つかればOK
                }
            }

            // フォントが設定できたらコンテキストに適用
            if font_loaded {
                cc.egui_ctx.set_fonts(fonts);
            }

            Box::new(SimulatorApp::default())
        }),
    )
}