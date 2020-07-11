use crate::cfg::SceneCfg;
use crate::color::Color;
use crate::pos::Pos;
use crate::pos::{crossprod_sign, polar, radians};
use crate::tesselation::Frame;
use rand::{rngs::ThreadRng, Rng};

pub struct Scene {
    bg: ColorItem,
    items: Vec<Box<dyn Contains>>,
}

impl Scene {
    pub fn new(cfg: &SceneCfg, rng: &mut ThreadRng) -> Self {
        Self {
            bg: cfg.choose_color(rng),
            items: cfg.create_items(rng),
        }
    }

    pub fn color(&self, p: Pos, rng: &mut ThreadRng) -> Color {
        for i in &self.items {
            if let Some(c) = i.contains(p, rng) {
                return c;
            }
        }
        self.bg.sample(rng)
    }
}

pub trait Contains {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color>;
}

pub struct ColorItem {
    pub shade: Color,
    pub deviation: i32,
    pub theme: Color,
    pub weight: i32,
}

impl ColorItem {
    pub fn sample(&self, rng: &mut ThreadRng) -> Color {
        self.shade
            .variate(rng, self.deviation)
            .meanpoint(self.theme, self.weight)
    }
}

pub struct Disc {
    pub center: Pos,
    pub radius: f64,
    pub color: ColorItem,
}

impl Disc {
    pub fn random(rng: &mut ThreadRng, f: &Frame, color: ColorItem, size_hint: f64) -> Self {
        let center = Pos::random(f, rng);
        let radius = (rng.gen::<f64>() * size_hint + 0.1) * (f.h.min(f.w) as f64);
        Self {
            center,
            radius,
            color,
        }
    }
}

impl Contains for Disc {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color> {
        if (self.center - p).dot_self() < self.radius.powi(2) {
            Some(self.color.sample(rng))
        } else {
            None
        }
    }
}

pub struct HalfPlane {
    pub limit: Pos,
    pub reference: Pos,
    pub color: ColorItem,
}

impl HalfPlane {
    pub fn random(rng: &mut ThreadRng, limit: Pos, indic: i32, var: i32, color: ColorItem) -> Self {
        Self {
            limit,
            reference: limit + polar(radians(rng.gen_range(indic - var, indic + var)), 100.),
            color,
        }
    }
}

impl Contains for HalfPlane {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color> {
        let dotprod = (p - self.limit).dot(self.reference - self.limit);
        if dotprod < 0. {
            Some(self.color.sample(rng))
        } else {
            None
        }
    }
}

pub struct Triangle {
    pub a: Pos,
    pub b: Pos,
    pub c: Pos,
    pub color: ColorItem,
}

impl Triangle {
    pub fn random(rng: &mut ThreadRng, circ: Disc) -> Self {
        let theta0 = rng.gen_range(0, 360);
        let theta1 = rng.gen_range(80, 150);
        let theta2 = rng.gen_range(80, 150);
        Self {
            a: circ.center + polar(radians(theta0), circ.radius),
            b: circ.center + polar(radians(theta0 + theta1), circ.radius),
            c: circ.center + polar(radians(theta0 + theta1 + theta2), circ.radius),
            color: circ.color,
        }
    }
}

impl Contains for Triangle {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color> {
        let d1 = crossprod_sign(p, self.a, self.b);
        let d2 = crossprod_sign(p, self.b, self.c);
        let d3 = crossprod_sign(p, self.c, self.a);
        let has_pos = d1 || d2 || d3;
        let has_neg = !(d1 && d2 && d3);
        if !(has_neg && has_pos) {
            Some(self.color.sample(rng))
        } else {
            None
        }
    }
}

pub struct Spiral {
    pub center: Pos,
    pub width: f64,
    pub color: ColorItem,
}

impl Spiral {
    pub fn random(rng: &mut ThreadRng, f: &Frame, color: ColorItem, width: f64) -> Self {
        Self {
            center: Pos::random(f, rng),
            width,
            color,
        }
    }
}

impl Contains for Spiral {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color> {
        let Pos(di, dj) = self.center - p;
        let theta = di.atan2(dj);
        let radius = (di.powi(2) + dj.powi(2)).sqrt() + theta / std::f64::consts::PI * self.width;
        if (radius / self.width).floor() as i32 % 2 == 0 {
            Some(self.color.sample(rng))
        } else {
            None
        }
    }
}

pub struct Stripe {
    limit: Pos,
    reference: Pos,
    color: ColorItem,
}

impl Stripe {
    pub fn random(rng: &mut ThreadRng, f: &Frame, color: ColorItem, width: f64) -> Self {
        let limit = Pos::random(f, rng);
        let reference = limit + polar(radians(rng.gen_range(0, 360)), width);
        Self {
            limit,
            reference,
            color,
        }
    }
}

impl Contains for Stripe {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color> {
        let dotprod1 = (p - self.limit).dot(self.reference - self.limit);
        let dotprod2 = (p - self.reference).dot(self.limit - self.reference);
        if dotprod1 > 0. && dotprod2 > 0. {
            Some(self.color.sample(rng))
        } else {
            None
        }
    }
}
