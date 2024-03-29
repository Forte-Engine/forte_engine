use cgmath::Quaternion;
use forte_engine::{component_app::EngineComponent, create_app, lights::{lights::LightUniform, LightEngine}, math::{quaternion::QuaternionExt, transforms::Transform}, models::{gltf::GLTFLoader, Model}, primitives::{cameras::Camera, transforms::TransformRaw}, run_app};
use gltf::Gltf;

pub struct TestComponent {
    camera: Camera,
    model: Model,
    instance_buffer: wgpu::Buffer
}

impl EngineComponent<(&mut RenderEngine, &mut LightEngine)> for TestComponent {

    fn create(engine: &mut RenderEngine) -> Self { 
        // generate camera
        let mut camera = Camera::new(
            &engine, 
            engine.config.width as f32 / engine.config.height as f32,
            45.0, 0.1, 100.0
        );
        camera.position = (0.0, 0.0, 5.0).into();
        camera.update(engine);

        // create instances
        let instances = vec![Transform {
            position: cgmath::Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            rotation: cgmath::Quaternion::euler_deg_z(0.0),
            scale: (1.0, 1.0, 1.0).into()
        }];

        let gltf = Gltf::from_slice(include_bytes!("mine.gltf.glb")).expect("Could not load binary gltf");
        let gltf = GLTFLoader::unpack_static_gltf(engine, gltf);

        Self {
            instance_buffer: TransformRaw::buffer_from_generic(engine, &instances),
            model: gltf,
            camera
        }
    }

    fn start(&mut self, (_, light_engine): (&mut RenderEngine, &mut LightEngine)) {
        light_engine.set_ambient_color([0.5, 0.5, 0.5]);
        light_engine.add_light(0, LightUniform::new(
            [
                5.0, 
                5.0, 
                5.0
            ], 
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 0.0],
            f32::MAX, 1.0, 1000.0
        ));
    }

    fn update(&mut self, (engine, _): (&mut RenderEngine, &mut LightEngine)) {
        // update rotation
        TransformRaw::update_buffer_generic(
            engine, &self.instance_buffer, 
            &[Transform {
                position: [0.0, -1.0, 0.0].into(),
                rotation: Quaternion::euler_deg(0.0, engine.time_since_start * 45.0, 0.0),
                ..Default::default()
            }]
        );
    }
    
    fn render<'rpass>(&'rpass mut self, engine: &'rpass RenderEngine, pass: &mut wgpu::RenderPass<'rpass>) {
        self.camera.bind(pass, engine, 0);
        self.model.draw(pass, &self.instance_buffer, 1);
    }

    fn exit(&mut self, _: (&mut RenderEngine, &mut LightEngine)) {}
}

create_app! {
    CLEAR_COLOR = wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },

    APP {
        light_engine: LightEngine[render_engine],
        test: TestComponent[render_engine, light_engine]
    },

    PASSES {
        0: {
            PARTS: [
                {
                    PIPELINE: "forte.gltf",
                    PREPARE: [light_engine],
                    RENDER: test,
                }
            ],
            DEPTH: true
        }
    }
}

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run() { println!("Starting run"); run_app::<App>().await }

fn main() { println!("Starting main"); pollster::block_on(run_app::<App>()) }