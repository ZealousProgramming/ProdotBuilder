use gdnative::api::{ArrayMesh, Mesh, MeshDataTool, ImmediateGeometry, MeshInstance};
use gdnative::prelude::*;

use std::collections::HashMap;

//use crate::prodot_utils::*;
#[derive(Copy, Clone, FromVariant, ToVariant)]
pub struct Face {
    pub tris_one: Vector3,
    pub tris_two: Vector3,
}

#[derive(NativeClass)]
#[inherit(MeshInstance)]
pub struct ProdotMesh {
    imm_geo: Option<Ref<ImmediateGeometry, Shared>>,
    vertices: TypedArray::<Vector3>,
    _uvs: TypedArray::<Vector2>,
    _normals: TypedArray::<Vector3>,
    _indices: TypedArray::<i32>,
    // Holds the indices of the 3 vertices that make up a face
    faces: Vec::<Face>,
    normal_color: Color,
    hover_color: Color,
    selected_color: Color,
    face_normal_color: Color,
    face_outline_color: Color,
    face_selected_color: Color,
    handle_x_color: Color,
    handle_x_color_hover: Color,
    handle_y_color: Color,
    handle_y_color_hover: Color,
    handle_z_color: Color,
    handle_z_color_hover: Color,
    handle_dist: f32,
}

#[methods]
impl ProdotMesh {
    pub fn new(_owner: TRef<MeshInstance>) -> Self {
        ProdotMesh {
            imm_geo: None,
            vertices: TypedArray::<Vector3>::new(),
            _uvs: TypedArray::<Vector2>::new(),
            _normals: TypedArray::<Vector3>::new(),
            _indices: TypedArray::<i32>::new(),
            faces: Vec::new(),
            normal_color: Color::rgba(0.4, 0.4, 0.4, 0.8),
            hover_color: Color::rgba(0.7, 0.7, 0.7, 1.0),
            selected_color: Color::rgba(0.2, 0.2, 0.2, 1.0),
            face_normal_color: Color::rgba(0.80784, 0.64314, 0.94510, 0.5),
            face_outline_color: Color::rgba(0.2, 0.2, 0.2, 1.0),
            face_selected_color: Color::rgba(0.16078, 0.54902, 0.96078, 0.7),
            //handle_x_color: Color::rgba(0.98039, 0.60784, 0.60784, 0.7),
            handle_x_color: Color::rgba(0.96078, 0.2, 0.31765, 0.7),
            //handle_x_color_hover: Color::rgba(0.98039, 0.60784, 0.60784, 1.0),
            handle_x_color_hover: Color::rgba(0.96078, 0.2, 0.31765, 1.0),
            //handle_y_color: Color::rgba(0.64706, 0.93725, 0.67451, 0.7),
            handle_y_color: Color::rgba(0.52941, 0.83922, 0.00784, 0.7),
            //handle_y_color_hover: Color::rgba(0.64706, 0.93725, 0.67451, 1.0),
            handle_y_color_hover: Color::rgba(0.52941, 0.83922, 0.00784, 1.0),
            //handle_z_color: Color::rgba(0.57255, 0.63529, 0.84314, 0.7),
            handle_z_color: Color::rgba(0.16078, 0.54902, 0.96078, 0.7),
            //handle_z_color_hover: Color::rgba(0.57255, 0.63529, 0.84314, 1.0),
            handle_z_color_hover: Color::rgba(0.16078, 0.54902, 0.96078, 1.0),
            handle_dist: 0.15,
        }
    }

    #[export]
    fn _enter_tree(&mut self, owner: TRef<MeshInstance>) {
        godot_print!("Prodot Mesh created!");
        
        self.imm_geo = unsafe {
            Some( 
                owner.
                    get_node("./ProdotIG")
                    .unwrap()
                    .assume_safe()
                    .cast::<ImmediateGeometry>()
                    .unwrap()
                    .claim()
            )
        };



    }

    #[export]
    fn _exit_tree(&self, _owner: TRef<MeshInstance>) {}
    
    #[export]
    pub fn clear(&mut self, _owner: TRef<MeshInstance>) {
        if let Some(geo_ref) = self.imm_geo {
            let geo = unsafe { geo_ref.assume_safe() };
            geo.clear();
        }
    }

    #[export]
    fn draw_gizmo_vertex(&mut self, owner: TRef<MeshInstance>, indices: Vec<i32>, hovering_gizmo_axis: Vector3) {
        // X plane
        let mut gizmo_dist: Vector3 = Vector3::new(self.handle_dist, 0.0, 0.0);
        
        let vertex: Vector3 = self.get_vertices_center(owner, indices);// self.vertices.get(selected_index);

        let half_height = 0.05;
        let half_depth = 0.05;
        let half_length = 0.05;
        
        let geo = unsafe { self.imm_geo.unwrap().assume_safe() };

        if hovering_gizmo_axis == Vector3::new(1.0, 0.0, 0.0) {
            geo.set_color(self.handle_x_color_hover);
        } else {
            geo.set_color(self.handle_x_color);
        }
        //bl
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(-half_length, -half_height, 0.0));
        //tl
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(-half_length, half_height, 0.0));

        //tr
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(half_length, half_height, 0.0));

        //tr
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(half_length, half_height, 0.0));

        //br
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(half_length, -half_height, 0.0));
        //bl
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(-half_length, -half_height, 0.0));

        // Y plane
        if hovering_gizmo_axis == Vector3::new(0.0, 1.0, 0.0) {
            geo.set_color(self.handle_y_color_hover);
        } else {
            geo.set_color(self.handle_y_color);
        }
        gizmo_dist = Vector3::new(0.0, self.handle_dist, 0.0);

        //bl
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(-half_length, -half_height, 0.0));
        //tl
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(-half_length, half_height, 0.0));

        //tr
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(half_length, half_height, 0.0));

        //tr
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(half_length, half_height, 0.0));

        //br
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(half_length, -half_height, 0.0));
        //bl
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(-half_length, -half_height, 0.0));
        
        // Z plane
        gizmo_dist = Vector3::new(0.0, 0.0, self.handle_dist);
        if hovering_gizmo_axis == Vector3::new(0.0, 0.0, 1.0) {
            geo.set_color(self.handle_z_color_hover);
        } else {
            geo.set_color(self.handle_z_color);
        }

        //bl
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(0.0, -half_height, half_depth));
        //tl
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(0.0, half_height, half_depth)); 

        //tr
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(0.0, half_height, -half_depth));

        //tr
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(0.0, half_height, -half_depth));

        //br
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(0.0, -half_height, -half_depth));
        //bl
        geo.add_vertex(vertex + gizmo_dist + Vector3::new(0.0, -half_height, half_depth));
        
    }

    #[export]
    pub fn draw_vertices(&mut self, owner: TRef<MeshInstance>, indices: Vec<i32>, hover_index: i32, hovering_gizmo_axis: Vector3) {
        if !self.vertices.is_empty() {
            let geo = unsafe { self.imm_geo.unwrap().assume_safe() };
            geo.clear();
            
            geo.begin(Mesh::PRIMITIVE_TRIANGLES, Null::null());
            let half_height = 0.05;
            let half_depth = 0.05;
            let half_length = 0.05;

            // Standard non-selected non-hovered
            geo.set_color(self.normal_color);

            for i in 0..self.vertices.len() {
                let vertex = self.vertices.get(i);
                if indices.contains(&(i as i32)) {
                    geo.set_color(self.selected_color);
                }else if hover_index == i {
                    geo.set_color(self.hover_color);
                }else {
                    geo.set_color(self.normal_color);
                }

                // Front Face 
                //bl
                geo.add_vertex(vertex + Vector3::new(-half_length, -half_height, half_depth));
                //tl
                geo.add_vertex(vertex + Vector3::new(-half_length, half_height, half_depth));

                //tr
                geo.add_vertex(vertex + Vector3::new(half_length, half_height, half_depth));

                //tr
                geo.add_vertex(vertex + Vector3::new(half_length, half_height, half_depth));

                //br
                geo.add_vertex(vertex + Vector3::new(half_length, -half_height, half_depth));
                //bl
                geo.add_vertex(vertex + Vector3::new(-half_length, -half_height, half_depth));
                
                // Back Face 
                //bl
                geo.add_vertex(vertex + Vector3::new(-half_length, -half_height, -half_depth));

                //tl
                geo.add_vertex(vertex + Vector3::new(-half_length, half_height, -half_depth));

                //tr
                geo.add_vertex(vertex + Vector3::new(half_length, half_height, -half_depth));

                //tr
                geo.add_vertex(vertex + Vector3::new(half_length, half_height, -half_depth));

                //br
                geo.add_vertex(vertex + Vector3::new(half_length, -half_height, -half_depth));
                //bl
                geo.add_vertex(vertex + Vector3::new(-half_length, -half_height, -half_depth));

                // Left Side
                //bl
                geo.add_vertex(vertex + Vector3::new( -half_length, -half_height, -half_depth));
                //tl
                geo.add_vertex(vertex + Vector3::new( -half_length, half_height, -half_depth));
                //tr
                geo.add_vertex(vertex + Vector3::new( -half_length, half_height, half_depth));
                //tr
                geo.add_vertex(vertex + Vector3::new( -half_length, half_height, half_depth));
                //br
                geo.add_vertex(vertex + Vector3::new( -half_length, -half_height, half_depth));
                //bl
                geo.add_vertex(vertex + Vector3::new( -half_length, -half_height, -half_depth));
   
                // Right Side
                //bl
                geo.add_vertex(vertex + Vector3::new( half_length, -half_height, -half_depth));
                //tl
                geo.add_vertex(vertex + Vector3::new( half_length, half_height, -half_depth));
                //tr
                geo.add_vertex(vertex + Vector3::new( half_length, half_height, half_depth));
                //tr
                geo.add_vertex(vertex + Vector3::new( half_length, half_height, half_depth));
                //br
                geo.add_vertex(vertex + Vector3::new( half_length, -half_height, half_depth));
                //bl
                geo.add_vertex(vertex + Vector3::new( half_length, -half_height, -half_depth));

                // Top Side
                //bl
                geo.add_vertex(vertex + Vector3::new( -half_length, half_height, half_depth));
                //tl
                geo.add_vertex(vertex + Vector3::new( -half_length, half_height, -half_depth));
                //tr
                geo.add_vertex(vertex + Vector3::new( half_length, half_height, -half_depth));
                //tr
                geo.add_vertex(vertex + Vector3::new( half_length, half_height, -half_depth));
                //br
                geo.add_vertex(vertex + Vector3::new( half_length, half_height, half_depth));
                //bl
                geo.add_vertex(vertex + Vector3::new( -half_length, half_height, half_depth));
                
                // Bottom Side
                //bl
                geo.add_vertex(vertex + Vector3::new( -half_length, -half_height, half_depth));
                //tl
                geo.add_vertex(vertex + Vector3::new( -half_length, -half_height, -half_depth));
                //tr
                geo.add_vertex(vertex + Vector3::new( half_length, -half_height, -half_depth));
                //tr
                geo.add_vertex(vertex + Vector3::new( half_length, -half_height, -half_depth));
                //br
                geo.add_vertex(vertex + Vector3::new( half_length, -half_height, half_depth));
                //bl
                geo.add_vertex(vertex + Vector3::new( -half_length, -half_height, half_depth));
            }

            if !indices.is_empty(){
                self.draw_gizmo_vertex(owner, indices, hovering_gizmo_axis);
            }
            geo.end();
        }

    }

    #[export]
    fn draw_gizmo_face(&mut self, owner: TRef<MeshInstance>, indices: Vec<i32>, hovering_gizmo_axis: Vector3) {
        // Draw center cube
        let center = self.get_face_center(owner, indices);

        let half_height = 0.05;
        let half_depth = 0.05;
        let half_length = 0.05;
        
        let geo = unsafe { self.imm_geo.unwrap().assume_safe() };

        if hovering_gizmo_axis == Vector3::new(1.0, 0.0, 0.0) {
            geo.set_color(self.handle_x_color_hover);
        } else {
            geo.set_color(self.handle_x_color);
        }

        // X plane
        let mut gizmo_dist: Vector3 = Vector3::new(self.handle_dist, 0.0, 0.0);
        //bl
        geo.add_vertex(center + gizmo_dist + Vector3::new(-half_length, -half_height, 0.0));
        //tl
        geo.add_vertex(center + gizmo_dist + Vector3::new(-half_length, half_height, 0.0));

        //tr
        geo.add_vertex(center + gizmo_dist + Vector3::new(half_length, half_height, 0.0));

        //tr
        geo.add_vertex(center + gizmo_dist + Vector3::new(half_length, half_height, 0.0));

        //br
        geo.add_vertex(center + gizmo_dist + Vector3::new(half_length, -half_height, 0.0));
        //bl
        geo.add_vertex(center + gizmo_dist + Vector3::new(-half_length, -half_height, 0.0));

        // Y plane
        if hovering_gizmo_axis == Vector3::new(0.0, 1.0, 0.0) {
            geo.set_color(self.handle_y_color_hover);
        } else {
            geo.set_color(self.handle_y_color);
        }
        gizmo_dist = Vector3::new(0.0, self.handle_dist, 0.0);

        //bl
        geo.add_vertex(center + gizmo_dist + Vector3::new(-half_length, -half_height, 0.0));
        //tl
        geo.add_vertex(center + gizmo_dist + Vector3::new(-half_length, half_height, 0.0));

        //tr
        geo.add_vertex(center + gizmo_dist + Vector3::new(half_length, half_height, 0.0));

        //tr
        geo.add_vertex(center + gizmo_dist + Vector3::new(half_length, half_height, 0.0));

        //br
        geo.add_vertex(center + gizmo_dist + Vector3::new(half_length, -half_height, 0.0));
        //bl
        geo.add_vertex(center + gizmo_dist + Vector3::new(-half_length, -half_height, 0.0));
        
        // Z plane
        gizmo_dist = Vector3::new(0.0, 0.0, self.handle_dist);
        if hovering_gizmo_axis == Vector3::new(0.0, 0.0, 1.0) {
            geo.set_color(self.handle_z_color_hover);
        } else {
            geo.set_color(self.handle_z_color);
        }

        //bl
        geo.add_vertex(center + gizmo_dist + Vector3::new(0.0, -half_height, half_depth));
        //tl
        geo.add_vertex(center + gizmo_dist + Vector3::new(0.0, half_height, half_depth)); 

        //tr
        geo.add_vertex(center + gizmo_dist + Vector3::new(0.0, half_height, -half_depth));

        //tr
        geo.add_vertex(center + gizmo_dist + Vector3::new(0.0, half_height, -half_depth));

        //br
        geo.add_vertex(center + gizmo_dist + Vector3::new(0.0, -half_height, -half_depth));
        //bl
        geo.add_vertex(center + gizmo_dist + Vector3::new(0.0, -half_height, half_depth));
        
    }

    #[export]
    pub fn draw_faces(&mut self, owner: TRef<MeshInstance>, indices: Vec<i32>, hover_index: i32, hovering_gizmo_axis: Vector3) {
        if !self.faces.is_empty() {
            let geo = unsafe { self.imm_geo.unwrap().assume_safe() };
            geo.clear();
            
            geo.begin(Mesh::PRIMITIVE_TRIANGLES, Null::null());
                        
            // Standard non-selected non-hovered
            geo.set_color(self.normal_color);
            
            //let outline_thickness = 0.005;
            let outline_thickness = 0.05;
            for i in 0..self.faces.len() {
                let face = self.faces[i];
                let mut draw: bool = false;
                if indices.contains(&(i as i32)){
                    geo.set_color(self.face_selected_color);
                    draw = true;
                }else if hover_index == i as i32{
                    geo.set_color(self.face_normal_color);
                    draw = true;
                }else {
                    geo.set_color(self.face_outline_color);
                }

                let vertex_one = self.vertices.get(face.tris_one.x as i32);
                let vertex_two = self.vertices.get(face.tris_one.y as i32);
                let vertex_three  = self.vertices.get(face.tris_one.z as i32);

                let vertex_four = self.vertices.get(face.tris_two.x as i32);
                let vertex_five = self.vertices.get(face.tris_two.y as i32);
                let vertex_six  = self.vertices.get(face.tris_two.z as i32);

                if draw {
                    geo.add_vertex(vertex_one);
                    geo.add_vertex(vertex_two);
                    geo.add_vertex(vertex_three);
                    
                    geo.add_vertex(vertex_four);
                    geo.add_vertex(vertex_five);
                    geo.add_vertex(vertex_six);
                }
                
                //self.draw_edge_lines(owner, vertex_one, vertex_two, vertex_three, outline_thickness);
                               
                // Draw center cube
                let center_pos = Vector3::new(
                    (vertex_one.x + vertex_two.x + vertex_three.x + vertex_four.x + vertex_five.x + vertex_six.x) / 6.0,
                    (vertex_one.y + vertex_two.y + vertex_three.y + vertex_four.y + vertex_five.y + vertex_six.y) / 6.0,
                    (vertex_one.z + vertex_two.z + vertex_three.z + vertex_four.z + vertex_five.z + vertex_six.z) / 6.0,
                );
                
                geo.set_color(self.face_outline_color);

                //front
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y - outline_thickness, center_pos.z + outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y + outline_thickness, center_pos.z + outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y + outline_thickness, center_pos.z + outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y + outline_thickness, center_pos.z + outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y - outline_thickness, center_pos.z + outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y - outline_thickness, center_pos.z + outline_thickness));
                //back
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y - outline_thickness, center_pos.z - outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y + outline_thickness, center_pos.z - outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y + outline_thickness, center_pos.z - outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y + outline_thickness, center_pos.z - outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y - outline_thickness, center_pos.z - outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y - outline_thickness, center_pos.z - outline_thickness));
                //right
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y - outline_thickness, center_pos.z - outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y + outline_thickness, center_pos.z - outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y + outline_thickness, center_pos.z + outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y + outline_thickness, center_pos.z + outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y - outline_thickness, center_pos.z + outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y - outline_thickness, center_pos.z - outline_thickness));
                //left
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y - outline_thickness, center_pos.z - outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y + outline_thickness, center_pos.z - outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y + outline_thickness, center_pos.z + outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y + outline_thickness, center_pos.z + outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y - outline_thickness, center_pos.z + outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y - outline_thickness, center_pos.z - outline_thickness));
                //top
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y + outline_thickness, center_pos.z + outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y + outline_thickness, center_pos.z - outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y + outline_thickness, center_pos.z - outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y + outline_thickness, center_pos.z - outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y + outline_thickness, center_pos.z + outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y + outline_thickness, center_pos.z + outline_thickness));

                //bottom
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y - outline_thickness, center_pos.z + outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y - outline_thickness, center_pos.z - outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y - outline_thickness, center_pos.z - outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y - outline_thickness, center_pos.z - outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x + outline_thickness, center_pos.y - outline_thickness, center_pos.z + outline_thickness));
                geo.add_vertex(Vector3::new(center_pos.x - outline_thickness, center_pos.y - outline_thickness, center_pos.z + outline_thickness));
                
            }

            if !indices.is_empty() {
                
                self.draw_gizmo_face(owner, indices, hovering_gizmo_axis);
            }

            geo.end();
        }

    }

    #[export]
    fn draw_edge_lines(&mut self, _owner: TRef<MeshInstance>, vertex_one: Vector3, vertex_two: Vector3, vertex_three: Vector3, outline_thickness: f32) {
        let geo = unsafe { self.imm_geo.unwrap().assume_safe() };
        // V1 -> V2
        
        // Front
        // bl
        geo.add_vertex(Vector3::new(vertex_one.x - outline_thickness, vertex_one.y, vertex_one.z + outline_thickness));
        // tl
        geo.add_vertex(Vector3::new(vertex_two.x - outline_thickness, vertex_two.y, vertex_two.z + outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_two.x, vertex_two.y, vertex_two.z + outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_two.x, vertex_two.y, vertex_two.z + outline_thickness));
        // br
        geo.add_vertex(Vector3::new(vertex_one.x, vertex_one.y, vertex_one.z + outline_thickness));
        // bl
        geo.add_vertex(Vector3::new(vertex_one.x - outline_thickness, vertex_one.y, vertex_one.z + outline_thickness));
        
        // Back
        // bl
        geo.add_vertex(Vector3::new(vertex_one.x - outline_thickness, vertex_one.y, vertex_one.z - outline_thickness));
        // tl
        geo.add_vertex(Vector3::new(vertex_two.x - outline_thickness, vertex_two.y, vertex_two.z - outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_two.x, vertex_two.y, vertex_two.z - outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_two.x, vertex_two.y, vertex_two.z - outline_thickness));
        // br
        geo.add_vertex(Vector3::new(vertex_one.x, vertex_one.y, vertex_one.z - outline_thickness));
        // bl
        geo.add_vertex(Vector3::new(vertex_one.x - outline_thickness, vertex_one.y, vertex_one.z - outline_thickness));
        
        // Right
        // bl
        geo.add_vertex(Vector3::new(vertex_one.x, vertex_one.y, vertex_one.z + outline_thickness));
        // tl
        geo.add_vertex(Vector3::new(vertex_two.x, vertex_two.y, vertex_two.z + outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_two.x, vertex_two.y, vertex_two.z - outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_two.x, vertex_two.y, vertex_two.z - outline_thickness));
        // br
        geo.add_vertex(Vector3::new(vertex_one.x, vertex_one.y, vertex_one.z - outline_thickness));
        // bl
        geo.add_vertex(Vector3::new(vertex_one.x, vertex_one.y, vertex_one.z + outline_thickness));
        
        // Left
        // bl
        geo.add_vertex(Vector3::new(vertex_one.x - outline_thickness, vertex_one.y, vertex_one.z + outline_thickness));
        // tl
        geo.add_vertex(Vector3::new(vertex_two.x - outline_thickness, vertex_two.y, vertex_two.z + outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_two.x - outline_thickness, vertex_two.y, vertex_two.z - outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_two.x - outline_thickness, vertex_two.y, vertex_two.z - outline_thickness));
        // br
        geo.add_vertex(Vector3::new(vertex_one.x - outline_thickness, vertex_one.y, vertex_one.z - outline_thickness));
        // bl
        geo.add_vertex(Vector3::new(vertex_one.x - outline_thickness, vertex_one.y, vertex_one.z + outline_thickness));
        
        
        // V2 -> V3
        
        geo.set_color(self.handle_x_color);
        // Front
        // bl 
        geo.add_vertex(Vector3::new(vertex_two.x, vertex_two.y, vertex_two.z + outline_thickness));
        // tl
        geo.add_vertex(Vector3::new(vertex_three.x, vertex_three.y, vertex_three.z + outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_three.x, vertex_three.y, vertex_three.z + outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_three.x, vertex_three.y, vertex_three.z + outline_thickness));
        // br
        geo.add_vertex(Vector3::new(vertex_two.x, vertex_two.y, vertex_two.z + outline_thickness));
        // bl
        geo.add_vertex(Vector3::new(vertex_two.x, vertex_two.y, vertex_two.z + outline_thickness));
        
        geo.set_color(self.face_outline_color);
        /*
        // Back
        // bl
        geo.add_vertex(Vector3::new(vertex_two.x - outline_thickness, vertex_two.y, vertex_two.z - outline_padding));
        // tl
        geo.add_vertex(Vector3::new(vertex_three.x - outline_thickness, vertex_three.y, vertex_three.z - outline_padding));
        // tr
        geo.add_vertex(Vector3::new(vertex_three.x, vertex_three.y, vertex_three.z - outline_padding));
        // tr
        geo.add_vertex(Vector3::new(vertex_three.x, vertex_three.y, vertex_three.z - outline_padding));
        // br
        geo.add_vertex(Vector3::new(vertex_two.x, vertex_two.y, vertex_two.z - outline_padding));
        // bl
        geo.add_vertex(Vector3::new(vertex_two.x - outline_thickness, vertex_two.y, vertex_two.z - outline_padding));
        */
        
        // Right
        // bl
        geo.add_vertex(Vector3::new(vertex_two.x, vertex_two.y, vertex_two.z + outline_thickness));
        // tl
        geo.add_vertex(Vector3::new(vertex_three.x, vertex_three.y, vertex_three.z + outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_three.x, vertex_three.y, vertex_three.z - outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_three.x, vertex_three.y, vertex_three.z - outline_thickness));
        // br
        geo.add_vertex(Vector3::new(vertex_two.x, vertex_two.y, vertex_two.z - outline_thickness));
        // bl
        geo.add_vertex(Vector3::new(vertex_two.x, vertex_two.y, vertex_two.z + outline_thickness));
        /*
        // Left
        // bl
        geo.add_vertex(Vector3::new(vertex_two.x - outline_thickness, vertex_two.y, vertex_two.z + outline_padding));
        // tl
        geo.add_vertex(Vector3::new(vertex_three.x - outline_thickness, vertex_three.y, vertex_three.z + outline_padding));
        // tr                              
        geo.add_vertex(Vector3::new(vertex_three.x - outline_thickness, vertex_three.y, vertex_three.z - outline_padding));
        // tr                              
        geo.add_vertex(Vector3::new(vertex_three.x - outline_thickness, vertex_three.y, vertex_three.z - outline_padding));
        // br
        geo.add_vertex(Vector3::new(vertex_two.x - outline_thickness, vertex_two.y, vertex_two.z - outline_padding));
        // bl
        geo.add_vertex(Vector3::new(vertex_two.x - outline_thickness, vertex_two.y, vertex_two.z + outline_padding));
        */
        // Top
        // bl
        geo.add_vertex(Vector3::new(vertex_two.x  , vertex_two.y + outline_thickness, vertex_two.z + outline_thickness));
        // tl
        geo.add_vertex(Vector3::new(vertex_three.x, vertex_three.y + outline_thickness, vertex_three.z + outline_thickness));
        // tr                              
        geo.add_vertex(Vector3::new(vertex_three.x , vertex_three.y + outline_thickness, vertex_three.z - outline_thickness));
        // tr                              
        geo.add_vertex(Vector3::new(vertex_three.x , vertex_three.y + outline_thickness, vertex_three.z - outline_thickness));
        // br
        geo.add_vertex(Vector3::new(vertex_two.x , vertex_two.y + outline_thickness, vertex_two.z - outline_thickness));
        // bl
        geo.add_vertex(Vector3::new(vertex_two.x, vertex_two.y + outline_thickness, vertex_two.z + outline_thickness));
                       
         
                       
        // V3 -> V1
        
        // Front
        // bl
        geo.add_vertex(Vector3::new(vertex_three.x - outline_thickness, vertex_three.y, vertex_three.z + outline_thickness));
        // tl
        geo.add_vertex(Vector3::new(vertex_one.x - outline_thickness, vertex_one.y, vertex_one.z + outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_one.x, vertex_one.y, vertex_one.z + outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_one.x, vertex_one.y, vertex_one.z + outline_thickness));
        // br
        geo.add_vertex(Vector3::new(vertex_three.x, vertex_three.y, vertex_three.z + outline_thickness));
        // bl
        geo.add_vertex(Vector3::new(vertex_three.x - outline_thickness, vertex_three.y, vertex_three.z + outline_thickness));
        
        // Back
        // bl
        geo.add_vertex(Vector3::new(vertex_three.x - outline_thickness, vertex_three.y, vertex_three.z - outline_thickness));
        // tl
        geo.add_vertex(Vector3::new(vertex_one.x - outline_thickness, vertex_one.y, vertex_one.z - outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_one.x, vertex_one.y, vertex_one.z - outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_one.x, vertex_one.y, vertex_one.z - outline_thickness));
        // br
        geo.add_vertex(Vector3::new(vertex_three.x, vertex_three.y, vertex_three.z - outline_thickness));
        // bl
        geo.add_vertex(Vector3::new(vertex_three.x - outline_thickness, vertex_three.y, vertex_three.z - outline_thickness));
        
        // Right
        // bl
        geo.add_vertex(Vector3::new(vertex_three.x, vertex_three.y, vertex_three.z + outline_thickness));
        // tl
        geo.add_vertex(Vector3::new(vertex_one.x, vertex_one.y, vertex_one.z + outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_one.x, vertex_one.y, vertex_one.z - outline_thickness));
        // tr
        geo.add_vertex(Vector3::new(vertex_one.x, vertex_one.y, vertex_one.z - outline_thickness));
        // br
        geo.add_vertex(Vector3::new(vertex_three.x, vertex_three.y, vertex_three.z - outline_thickness));
        // bl
        geo.add_vertex(Vector3::new(vertex_three.x, vertex_three.y, vertex_three.z + outline_thickness));
        
        // Left
        // bl
        geo.add_vertex(Vector3::new(vertex_three.x - outline_thickness, vertex_three.y, vertex_three.z + outline_thickness));
        // tl
        geo.add_vertex(Vector3::new(vertex_one.x - outline_thickness, vertex_one.y, vertex_one.z + outline_thickness));
        // tr                              one
        geo.add_vertex(Vector3::new(vertex_one.x - outline_thickness, vertex_one.y, vertex_one.z - outline_thickness));
        // tr                              one
        geo.add_vertex(Vector3::new(vertex_one.x - outline_thickness, vertex_one.y, vertex_one.z - outline_thickness));
        // br
        geo.add_vertex(Vector3::new(vertex_three.x - outline_thickness, vertex_three.y, vertex_three.z - outline_thickness));
        // bl
        geo.add_vertex(Vector3::new(vertex_three.x - outline_thickness, vertex_three.y, vertex_three.z + outline_thickness));
    }

    #[export]
    pub fn get_vertices(&mut self, _owner: TRef<MeshInstance>) -> TypedArray<Vector3> {
        self.vertices.clone()
    }

    #[export]
    pub fn get_vertex(&mut self, _owner: TRef<MeshInstance>, index: i32) -> Vector3 {
        self.vertices.get(index)
    }

    #[export]
    pub fn set_vertices(&mut self, _owner: TRef<MeshInstance>, vertices: TypedArray<Vector3>) {
        self.vertices = vertices;
    }

    //pub fn set_vertex(&mut self, owner: TRef<MeshInstance>, index: i32, position: Vector3) {
    pub fn set_vertex(&mut self, owner: TRef<MeshInstance>, updated_vertices: HashMap<i32, Vector3>) {
        //self.vertices.set(index, position);
        //self.update_mesh_vertex(owner, index, position);
        self.update_mesh_vertex(owner, updated_vertices);
    }

    pub fn get_vertices_center(&mut self, owner: TRef<MeshInstance>, indices: Vec<i32>) -> Vector3 {
        let mesh_pos = owner.global_transform().origin;
        let num_indices = indices.len();
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut z: f32 = 0.0;

        for index in indices {
            let vertex = self.vertices.get(index);
            x += vertex.x;
            y += vertex.y;
            z += vertex.z;
        }


        Vector3::new(
            x / num_indices as f32,
            y / num_indices as f32,
            z / num_indices as f32
        )
    }

    //fn update_mesh_vertex(&mut self, owner: TRef<MeshInstance>, index: i32, position: Vector3) {
    fn update_mesh_vertex(&mut self, owner: TRef<MeshInstance>, updated_vertices: HashMap<i32, Vector3>) {
        //let mesh_pos = owner.global_transform().origin;
        let mesh_ref = owner.mesh().unwrap();
        let mesh = unsafe { mesh_ref.assume_safe() };
        let mesh_array =  mesh.cast::<ArrayMesh>().unwrap();
        let mesh_tool = MeshDataTool::new();
        mesh_tool.create_from_surface(mesh_array, 0).expect("[Prodot Mesh]: Failed to create mesh from surface!");
        
        for (index, position) in updated_vertices {
            self.vertices.set(index, position);
            mesh_tool.set_vertex(index as i64, position);
        }
        mesh_array.surface_remove(0);
        mesh_tool.commit_to_surface(mesh_array).expect("[Prodot Mesh]: Failed to commit mesh array to surface!");
    }

    fn update_mesh_face(&mut self, owner: TRef<MeshInstance>, updated_faces: HashMap<i32, Vector3>) {
        let mesh_ref = owner.mesh().unwrap();
        let mesh = unsafe { mesh_ref.assume_safe() };
        let mesh_array =  mesh.cast::<ArrayMesh>().unwrap();
        let mesh_tool = MeshDataTool::new();
        mesh_tool.create_from_surface(mesh_array, 0).expect("[Prodot Mesh]: Failed to create mesh from surface!");
        
        for (index, _position) in updated_faces {
        //let indices = Vec::<i32>::new();
        //indices.push(index);
        //let center = self.get_face_center(owner, indices);
            let face = self.faces[index as usize];
            let vertex_one = self.vertices.get(face.tris_one.x as i32);
            let vertex_two = self.vertices.get(face.tris_one.y as i32);
            let vertex_three = self.vertices.get(face.tris_one.z as i32);
            let vertex_four = self.vertices.get(face.tris_two.x as i32);
            let vertex_five = self.vertices.get(face.tris_two.y as i32);
            let vertex_six =  self.vertices.get(face.tris_two.z as i32);
            
            self.vertices.set(face.tris_one.x as i32, vertex_one);
            self.vertices.set(face.tris_one.y as i32, vertex_two);
            self.vertices.set(face.tris_one.z as i32, vertex_three);
            self.vertices.set(face.tris_two.x as i32, vertex_four);
            self.vertices.set(face.tris_two.y as i32, vertex_five);
            self.vertices.set(face.tris_two.z as i32, vertex_six);

            //mesh_tool.set_vertex(face.tris_one.x as i64, vertex_one);
            //mesh_tool.set_vertex(face.tris_one.y as i64, vertex_two);
            //mesh_tool.set_vertex(face.tris_one.z as i64, vertex_three);
            //mesh_tool.set_vertex(face.tris_two.x as i64, vertex_four);
            //mesh_tool.set_vertex(face.tris_two.y as i64, vertex_five);
            //mesh_tool.set_vertex(face.tris_two.z as i64, vertex_six);
        }
        mesh_array.surface_remove(0);
        mesh_tool.commit_to_surface(mesh_array).expect("[Prodot Mesh]: Failed to commit mesh array to surface!");
    }

    #[export]
    pub fn get_faces(&mut self, _owner: TRef<MeshInstance>) -> Vec<Face> {
        self.faces.clone()
    }

    #[export]
    pub fn set_faces(&mut self, _owner: TRef<MeshInstance>, faces: Vec<Face>) {
        self.faces = faces;
    }

    pub fn set_face(&mut self, owner: TRef<MeshInstance>, updated_faces: HashMap<i32, Vector3>) {
        self.update_mesh_face(owner, updated_faces);
    }

    #[export]
    pub fn get_face_center(&mut self, owner: TRef<MeshInstance>, indices: Vec<i32>) -> Vector3 {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut z: f32 = 0.0;
        
        let mesh_pos = owner.global_transform().origin;
        let num_indices = indices.len();
        
        for i in 0..num_indices {
            let face = self.faces[i];
            let vertex_one = mesh_pos + self.vertices.get(face.tris_one.x as i32);
            let vertex_two = mesh_pos + self.vertices.get(face.tris_one.y as i32);
            let vertex_three = mesh_pos + self.vertices.get(face.tris_one.z as i32);

            let vertex_four = mesh_pos + self.vertices.get(face.tris_two.x as i32);
            let vertex_five = mesh_pos + self.vertices.get(face.tris_two.y as i32);
            let vertex_six =  mesh_pos + self.vertices.get(face.tris_two.z as i32);

            x += vertex_one.x + vertex_two.x + vertex_three.x + vertex_four.x + vertex_five.x + vertex_six.x;
            y += vertex_one.y + vertex_two.y + vertex_three.y + vertex_four.y + vertex_five.y + vertex_six.y;
            z += vertex_one.z + vertex_two.z + vertex_three.z + vertex_four.z + vertex_five.z + vertex_six.z;
        }

        Vector3::new(
            x / ( 6.0 * num_indices as f32),
            y / ( 6.0 * num_indices as f32),
            z / ( 6.0 * num_indices as f32),
        )

    }


}
