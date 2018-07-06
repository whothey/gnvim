use pango::FontDescription;
use pango;
use pangocairo;
use cairo;
use gtk::{DrawingArea};

use cairo::prelude::*;
use gtk::prelude::*;
use pango::prelude::*;

use ui::color::{Color, Highlight};

pub struct Context {
    pub cairo_context: cairo::Context,
    pub pango_context: pango::Context,
    pub font_desc: FontDescription,
    pub cell_metrics: CellMetrics,
    // im_context

    // row, col
    pub cursor: (u64, u64),

    pub default_fg: Color,
    pub default_bg: Color,
    pub default_sp: Color,

    pub current_hl: Highlight,
}

impl Context {
    pub fn new(da: &DrawingArea) -> Self {
        let win = da.get_window().unwrap();
        let w = da.get_allocated_width();
        let h = da.get_allocated_width();
        let surface = win.create_similar_surface(cairo::Content::Color, w, h).unwrap();

        let cairo_context = cairo::Context::new(&surface);
        let pango_context = pangocairo::functions::create_context(&cairo_context).unwrap();

        let font_desc = FontDescription::from_string("Monospace 12");
        pango_context.set_font_description(&font_desc);

        let mut cell_metrics = CellMetrics::default();
        cell_metrics.update(&pango_context, &font_desc);

        //let row_count = h / cell_metrics.height as i32;
        //let col_count = w / cell_metrics.width as i32;
        //let mut rows = Vec::with_capacity(row_count as usize);
        //for r in [0..row_count].iter() {
            //let mut row = Vec::with_capacity(col_count as usize);
            //for c in [0..col_count].iter() {
                //row.push(Cell::default());
            //}
            //rows.push(row);
        //}

        Context {
            cairo_context,
            pango_context,
            font_desc,
            cell_metrics,
            //rows,
            cursor: (0, 0),

            default_fg: Color::default(),
            default_bg: Color::default(),
            default_sp: Color::default(),
            current_hl: Highlight::default(),
        }
    }

    pub fn update(&mut self, da: &DrawingArea) {
        let win = da.get_window().unwrap();
        let w = da.get_allocated_width();
        let h = da.get_allocated_width();
        let surface = win.create_similar_surface(cairo::Content::Color, w, h).unwrap();
        let ctx = cairo::Context::new(&surface);

        let s = self.cairo_context.get_target();
        self.cairo_context.save();
        ctx.set_source_surface(&s, 0.0, 0.0);
        ctx.set_operator(cairo::Operator::Source);
        ctx.paint();
        self.cairo_context.restore();

        let pctx = pangocairo::functions::create_context(&ctx).unwrap();
        pctx.set_font_description(&self.font_desc);

        self.cairo_context = ctx;
        self.pango_context = pctx;

        self.cell_metrics.update(&self.pango_context, &self.font_desc);
    }
}

#[derive(Default)]
pub struct CellMetrics {
    pub height: f64,
    pub width: f64,
    pub ascent: f64,
    pub decent: f64,
}

impl CellMetrics {
    pub fn update(&mut self, ctx: &pango::Context, desc: &FontDescription) {
        let fm = ctx.get_metrics(Some(desc), None).unwrap();
        self.ascent = fm.get_ascent() as f64 / pango::SCALE as f64;
        self.decent = fm.get_descent() as f64 / pango::SCALE as f64;
        self.height = self.ascent + self.decent;
        self.width = (fm.get_approximate_digit_width() / pango::SCALE) as f64;
    }
}