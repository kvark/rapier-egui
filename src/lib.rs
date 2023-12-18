/// Structure to hold the Rapier-Egui inspector state.
pub struct Inspector {
    selected_rigid_body: Option<rapier3d::dynamics::RigidBodyHandle>,
    selected_collider: Option<rapier3d::geometry::ColliderHandle>,
}

trait UiValue {
    fn value(&mut self, v: f32);
    fn value_vec3(&mut self, v3: &nalgebra::Vector3<f32>) {
        for &v in v3.as_slice() {
            self.value(v);
        }
    }
}
impl UiValue for egui::Ui {
    fn value(&mut self, v: f32) {
        self.colored_label(egui::Color32::WHITE, format!("{v:.1}"));
    }
}

impl Inspector {
    /// Create a new rapier inspector.
    pub fn new() -> Self {
        Inspector {
            selected_rigid_body: None,
            selected_collider: None,
        }
    }

    /// Reset all selections.
    pub fn reset_selection(&mut self) {
        self.selected_rigid_body = None;
        self.selected_collider = None;
    }

    /// Produce a sequence of checkboxes for various render debug modes.
    pub fn populate_debug_render(
        &mut self,
        ui: &mut egui::Ui,
        debug_render_mode: &mut rapier3d::pipeline::DebugRenderMode,
    ) {
        let all_bits = rapier3d::pipeline::DebugRenderMode::all().bits();
        for bit_pos in 0..=all_bits.ilog2() {
            let flag = match rapier3d::pipeline::DebugRenderMode::from_bits(1 << bit_pos) {
                Some(flag) => flag,
                None => continue,
            };
            let mut enabled = debug_render_mode.contains(flag);
            ui.checkbox(&mut enabled, format!("{flag:?}"));
            debug_render_mode.set(flag, enabled);
        }
    }

    /// Produce a list of rigid bodies and their colliders.
    pub fn populate_objects(
        &mut self,
        ui: &mut egui::Ui,
        rigid_bodies: &mut rapier3d::dynamics::RigidBodySet,
        colliders: &mut rapier3d::geometry::ColliderSet,
    ) {
        for (handle, _object) in rigid_bodies.iter() {
            ui.selectable_value(&mut self.selected_rigid_body, Some(handle), "rigid body");
        }

        if let Some(rb_handle) = self.selected_rigid_body {
            let rigid_body = &rigid_bodies[rb_handle];
            if ui.button("Unselect").clicked() {
                self.reset_selection();
            }
            egui::CollapsingHeader::new("Stats")
                .default_open(false)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("Position:"));
                        ui.value_vec3(&rigid_body.translation());
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("Linear velocity:"));
                        ui.value_vec3(&rigid_body.linvel());
                        ui.label(format!("Damping:"));
                        ui.value(rigid_body.linear_damping());
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("Angular velocity:"));
                        ui.value_vec3(&rigid_body.angvel());
                        ui.label(format!("Damping:"));
                        ui.value(rigid_body.angular_damping());
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("Kinematic energy:"));
                        ui.value(rigid_body.kinetic_energy());
                    });
                });
            ui.heading("Colliders");
            for &collider in rigid_body.colliders() {
                let name = "collider";
                ui.selectable_value(&mut self.selected_collider, Some(collider), name);
            }
        }

        if let Some(collider_handle) = self.selected_collider {
            ui.heading("Properties");
            let collider = colliders.get_mut(collider_handle).unwrap();
            let mut mass = collider.mass();
            if ui
                .add(
                    egui::DragValue::new(&mut mass)
                        .prefix("Mass: ")
                        .clamp_range(0.0..=1e6),
                )
                .changed()
            {
                collider.set_mass(mass);
            }
            let mut friction = collider.friction();
            if ui
                .add(
                    egui::DragValue::new(&mut friction)
                        .prefix("Friction: ")
                        .clamp_range(0.0..=2.0),
                )
                .changed()
            {
                collider.set_friction(friction);
            }
            let mut restitution = collider.restitution();
            if ui
                .add(
                    egui::DragValue::new(&mut restitution)
                        .prefix("Restituion: ")
                        .clamp_range(0.0..=1.0),
                )
                .changed()
            {
                collider.set_restitution(restitution);
            }
        }
    }
}
