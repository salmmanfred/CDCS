use glm::*;

pub struct CameraState {
    position: Vec3,
    screen_size: (f32, f32),
    fov: f32,
    near_plane: f32,
    far_plane: f32,
    dragging: bool,
    last_drag_pos: (f32, f32),
}

fn matrix_to_array(mat: Mat4) -> [[f32; 4]; 4] {
    [
        [mat[0], mat[1], mat[2], mat[3]],
        [mat[4], mat[5], mat[6], mat[7]],
        [mat[8], mat[9], mat[10], mat[11]],
        [mat[12], mat[13], mat[14], mat[15]],
    ]
}

impl CameraState {
    pub fn new(screen_size: (i32, i32)) -> CameraState {
        CameraState {
            screen_size: (screen_size.0 as f32, screen_size.1 as f32),
            position: glm::Vec3::new(0.5, 0.5, 0.2),
            fov: 75.0,
            near_plane: 0.001,
            far_plane: 10.0,
            dragging: false,
            last_drag_pos: (0.0, 0.0),
        }
    }

    pub fn scroll(&mut self, scroll : f32) {
        self.position.z = glm::clamp_scalar(self.position.z * scroll, 0.01, 1.0);
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        return matrix_to_array(self.perspective());
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        return matrix_to_array(self.view());
    }

    fn perspective(&self) -> glm::Mat4 {
        let aspect_ratio = self.screen_size.0 / self.screen_size.1;
        let fov_radians = self.fov * std::f32::consts::PI / 180.0;
        return glm::perspective(aspect_ratio, fov_radians, self.near_plane, self.far_plane);
    }

    fn view(&self) -> glm::Mat4 {
        let mut look_at = self.position;
        look_at.z = 0.0;
        // Code to see map from the side
        // if self.position.z < 1.0 {
        //     look_at.y += 0.06 * (1.0 - self.position.z);
        // }
        let up_vector = glm::vec3(0.0, 1.0, 0.0);
        return glm::look_at(&self.position, &look_at, &up_vector);
    }

    // Moves the map by dragging the mouse
    pub fn update(&mut self, mouse_pos: (i32, i32), dragging: bool) {
        if dragging {
            if !self.dragging
                && mouse_pos.0 > 0
                && mouse_pos.0 < self.screen_size.0 as i32
                && mouse_pos.1 > 0
                && mouse_pos.1 < self.screen_size.1 as i32
            {
                self.dragging = true;
                self.last_drag_pos = self.get_map_pos(mouse_pos);
            }
        }
        else {
            self.dragging = false;
        }
        if self.dragging {
            let map_pos = self.get_map_pos(mouse_pos);
            self.position.x += self.last_drag_pos.0 - map_pos.0;
            self.position.x = clamp_scalar(self.position.x, 0.0, 2.0);
            self.position.y += self.last_drag_pos.1 - map_pos.1;
            self.position.y = clamp_scalar(self.position.y, 0.0, 1.0);
        }
    }

    // Gets the the world pos on the map plane
    pub fn get_map_pos(&self, mouse_pos: (i32, i32)) -> (f32, f32) {
        let view = self.view();
        let perspective = self.perspective();

        let mouse_x = mouse_pos.0 as f32;
        let mouse_y = self.screen_size.1 - 1.0 - mouse_pos.1 as f32;

        let world_space_near = glm::unproject(
            &glm::Vec3::new(mouse_x, mouse_y, 0.0),
            &view,
            &perspective,
            glm::Vec4::new(0., 0., self.screen_size.0, self.screen_size.1),
        ) as Vec3;

        let world_space_far = glm::unproject(
            &glm::Vec3::new(mouse_x, mouse_y, 1.0),
            &view,
            &perspective,
            glm::vec4(0., 0., self.screen_size.0, self.screen_size.1),
        ) as Vec3;

        let ray_origin = world_space_near;
        let ray_direction = glm::normalize(&(world_space_far - world_space_near));

        let mut distance = 0.0;

        let center = glm::Vec3::new(0., 0., 0.);
        let normal = glm::Vec3::new(0., 0., 1.);
        let denom = glm::dot(&normal, &ray_direction);
        if f32::abs(denom) > 0.0001 {
            let t = glm::dot(&(center - ray_origin), &normal) / denom;
            if t >= 0.0 {
                distance = t;
            }
        }
        let pos = ray_origin + ray_direction * distance;

        return (pos.x, pos.y);
    }
}
