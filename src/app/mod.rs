use rand::Rng;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use std::f64::consts::PI;

use crate::model::{Sector, System, World};
use array2d::Array2D;

const HORIZONTAL_OFFSET: f64 = 0.5;
const VERTICAL_OFFSET: f64 = 0.25;
const STAGGER_OFFSET: f64 = 0.5;
const FIELD_DENSITY: f64 = 1.6;

type Map = Array2D<Option<System>>;

pub struct App {
    world: World,
    map: Map,
    starfield: Vec<(f64, f64)>,
    mode: SelectionMode,
    cursor: (u8, u8),
}

impl App {
    pub fn new(world: World) -> Self {
        let rows = world.sector().rows.into();
        let columns = world.sector().columns.into();

        let mut map = Array2D::filled_with(None, columns, rows);

        for s in world.systems() {
            dbg!(&s);
            map[((s.x - 1).into(), (s.y - 1).into())] = Some(s.clone());
        }

        for b in world.black_holes() {
            map[((b.x - 1).into(), (b.y - 1).into())] = Some(b.clone());
        }

        let mut rng: Pcg64 = Seeder::from(&world.sector().name).make_rng();

        // TODO: seed from app invariant, like name
        let mut starfield =
            Vec::with_capacity(((rows * columns) as f64 * FIELD_DENSITY).ceil() as usize);
        for _ in 0..starfield.capacity() {
            starfield.push((
                rng.gen_range(-1.0..=(rows as f64 + 1.0)),
                rng.gen_range(-1.0..=(columns as f64 + 1.0)),
            ));
        }

        Self {
            world,
            map,
            starfield,
            mode: SelectionMode::MAP,
            cursor: (1, 1),
        }
    }

    fn hex_to_coords(&self, hex: (u8, u8)) -> (f64, f64) {
        (
            hex.0 as f64 - HORIZONTAL_OFFSET,
            (self.sector().rows - hex.1) as f64
                + ((hex.0 % 2) as f64 * STAGGER_OFFSET + VERTICAL_OFFSET),
        )
    }

    fn coords_to_hex(&self, coords: (f64, f64)) -> (u8, u8) {
        (
            (coords.0 + HORIZONTAL_OFFSET).abs().round() as u8,
            (coords.1.floor() as u8 - self.sector().rows),
        )
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn starfield(&self) -> &Vec<(f64, f64)> {
        &self.starfield
    }

    pub fn system_coords(&self) -> Vec<(f64, f64)> {
        self.world
            .systems()
            .iter()
            .map(|s| self.hex_to_coords(s.hex()))
            .collect()
    }

    pub fn black_hole_coords(&self) -> Vec<(f64, f64)> {
        self.world
            .black_holes()
            .iter()
            .map(|b| self.hex_to_coords(b.hex()))
            .collect()
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn sector(&self) -> &Sector {
        self.world.sector()
    }

    pub fn cursor(&self) -> (u8, u8) {
        self.cursor.into()
    }

    pub fn selection_mode(&self) -> SelectionMode {
        self.mode.clone()
    }

    pub fn toggle_selection_mode(&mut self) {
        match self.mode {
            SelectionMode::MAP => self.mode = SelectionMode::SYSTEM,
            SelectionMode::SYSTEM => self.mode = SelectionMode::OBJECTS,
            SelectionMode::OBJECTS => self.mode = SelectionMode::MAP,
        }
    }

    pub fn move_selection(&mut self, change: SelectionChange) {
        if self.selection_mode() == SelectionMode::MAP {
            match change {
                SelectionChange::UP => {
                    if self.cursor.1 > 1 {
                        self.cursor.1 -= 1;
                    }
                }
                SelectionChange::DOWN => {
                    if self.cursor.1 < self.sector().rows {
                        self.cursor.1 += 1;
                    }
                }
                SelectionChange::LEFT => {
                    if self.cursor.0 > 1 {
                        self.cursor.0 -= 1;
                    }
                }
                SelectionChange::RIGHT => {
                    if self.cursor.0 < self.sector().columns {
                        self.cursor.0 += 1;
                    }
                }
            }
        }
    }

    pub fn selection(&self, cursor: SelectionCursor) -> Vec<(f64, f64)> {
        if cursor == SelectionCursor::HEX {
            let (x, y) = self.hex_to_coords(self.cursor);
            let short_offset = (PI / 6.0).tan();
            let long_offset = (PI / 6.0).cos();
            vec![
                (x - short_offset, y + 0.4),
                (x + short_offset, y + 0.4),
                (x + long_offset, y),
                (x + short_offset, y - 0.4),
                (x - short_offset, y - 0.4),
                (x - long_offset, y),
                (x - short_offset, y + 0.4),
            ]
        } else {
            vec![self.hex_to_coords(self.cursor)]
        }
    }

    pub fn selected_system(&self) -> Option<&System> {
        self.map
            .get((self.cursor.0 - 1).into(), (self.cursor.1 - 1).into())
            .unwrap()
            .as_ref()
    }
}

#[derive(PartialEq, Clone)]
pub enum SelectionMode {
    MAP,
    SYSTEM,
    OBJECTS,
}

#[derive(PartialEq, Clone)]
pub enum SelectionChange {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(PartialEq, Clone)]
pub enum SelectionCursor {
    BLOCK,
    HEX,
}
