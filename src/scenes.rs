use std::{fs::File, io::BufReader, path::Path, sync::Arc};

use crate::{
    camera::Camera, material::Material, object::Object, texture::Texture,
    utilities::vector3::Vector3, bvh::BVHNode
};
use image::codecs::hdr::HdrDecoder;
use rand::Rng;
#[allow(dead_code)]
pub enum Scenes {
    Basic,
    BasicChecker,
    HDRITest,
    RectangleLight,
    CornellBox,
    Volumes,
}

impl Scenes {
    pub fn get(&self, width: f64, height: f64) -> (Vec<Object>, Camera, Vector3<f64>) {
                   let mut rng = rand::thread_rng();
 match self {
            Self::Basic => {
                let look_from = Vector3::new(13.0, 2.0, 3.0);
                let look_at = Vector3::new(0.0, 0.0, 0.0);
                let vup = Vector3::new(0.0, 1.0, 0.0);
                let dist_to_focus = 10.0;
                let aperture = 0.1;

                let camera = Camera::new(
                    look_from,
                    look_at,
                    vup,
                    20.0,
                    width / height,
                    aperture,
                    dist_to_focus,
                );
                let material_ground = Material::Lambertian {
                    albedo: Vector3::new(0.5, 0.5, 0.5),
                };
                let mut objects = vec![Object::build_sphere(
                    Vector3::new(0.0, -1000.0, -1.0),
                    1000.0,
                    material_ground,
                )];

                for a in -11..11 {
                    for b in -11..11 {
                        let x = a as f64;
                        let y = b as f64;
                        let choose_material = rng.gen_range(1..100);
                        let center = Vector3::new(
                            x + 0.9 * rng.gen::<f64>(),
                            0.2,
                            y + 0.9 * rng.gen::<f64>(),
                        );

                        if choose_material < 80 {
                            // diffuse
                            let albedo = Vector3::random_vec(0.0, 1.0, &mut rng)
                                * Vector3::random_vec(0.0, 1.0, &mut rng);
                            let material = Material::Lambertian { albedo };
                            objects.push(Object::build_sphere(center, 0.2, material));
                        } else if choose_material < 95 {
                            //metal
                            let albedo = Vector3::random_vec(0.5, 1.0, &mut rng);
                            let fuzz = rng.gen_range(0.0..0.5);
                            let material = Material::Metal { albedo, fuzz };
                            objects.push(Object::build_sphere(center, 0.2, material));
                        } else {
                            //glass
                            let material = Material::Dielectric {
                                index_of_refraction: 1.5,
                            };
                            objects.push(Object::build_sphere(center, 0.2, material));
                        }
                    }
                }

                let material1 = Material::Dielectric {
                    index_of_refraction: 1.5,
                };
                let material2 = Material::Lambertian {
                    albedo: Vector3::new(0.4, 0.2, 0.1),
                };
                let material3 = Material::Metal {
                    albedo: Vector3::new(0.7, 0.6, 0.5),
                    fuzz: 0.0,
                };

                objects.push(Object::build_sphere(
                    Vector3::new(0.0, 1.0, 0.0),
                    1.0,
                    material1,
                ));
                objects.push(Object::build_sphere(
                    Vector3::new(-4.0, 1.0, 0.0),
                    1.0,
                    material2,
                ));
                objects.push(Object::build_sphere(
                    Vector3::new(4.0, 1.0, 0.0),
                    1.0,
                    material3,
                ));
                (objects, camera, Vector3::new(0.5, 0.7, 1.0))
            }

            Self::BasicChecker => {
                let mut rng = rand::thread_rng();
                let look_from = Vector3::new(13.0, 2.0, 3.0);
                let look_at = Vector3::new(0.0, 0.0, 0.0);
                let vup = Vector3::new(0.0, 1.0, 0.0);
                let dist_to_focus = 10.0;
                let aperture = 0.1;

                let camera = Camera::new(
                    look_from,
                    look_at,
                    vup,
                    20.0,
                    width / height,
                    aperture,
                    dist_to_focus,
                );
                let material_ground = Material::TexturedLambertian {
                    texture: Texture::Checker {
                        color1: Vector3::new(0.2, 0.3, 0.1),
                        color2: Vector3::new(0.9, 0.9, 0.9),
                    },
                };
                let mut objects = vec![Object::build_sphere(
                    Vector3::new(0.0, -1000.0, -1.0),
                    1000.0,
                    material_ground,
                )];

                for a in -11..11 {
                    for b in -11..11 {
                        let x = a as f64;
                        let y = b as f64;
                        let choose_material = rng.gen_range(1..100);
                        let center = Vector3::new(
                            x + 0.9 * rng.gen::<f64>(),
                            0.2,
                            y + 0.9 * rng.gen::<f64>(),
                        );

                        if choose_material < 80 {
                            // diffuse
                            let albedo = Vector3::random_vec(0.0, 1.0, &mut rng)
                                * Vector3::random_vec(0.0, 1.0, &mut rng);
                            let material = Material::Lambertian { albedo };
                            objects.push(Object::build_sphere(center, 0.2, material));
                        } else if choose_material < 95 {
                            //metal
                            let albedo = Vector3::random_vec(0.5, 1.0, &mut rng);
                            let fuzz = rng.gen_range(0.0..0.5);
                            let material = Material::Metal { albedo, fuzz };
                            objects.push(Object::build_sphere(center, 0.2, material));
                        } else {
                            //glass
                            let material = Material::Dielectric {
                                index_of_refraction: 1.5,
                            };
                            objects.push(Object::build_sphere(center, 0.2, material));
                        }
                    }
                }

                let material1 = Material::Dielectric {
                    index_of_refraction: 1.5,
                };
                let material2 = Material::Lambertian {
                    albedo: Vector3::new(0.4, 0.2, 0.1),
                };
                let material3 = Material::Metal {
                    albedo: Vector3::new(0.7, 0.6, 0.5),
                    fuzz: 0.0,
                };

                objects.push(Object::build_sphere(
                    Vector3::new(0.0, 1.0, 0.0),
                    1.0,
                    material1,
                ));
                objects.push(Object::build_sphere(
                    Vector3::new(-4.0, 1.0, 0.0),
                    1.0,
                    material2,
                ));
                objects.push(Object::build_sphere(
                    Vector3::new(4.0, 1.0, 0.0),
                    1.0,
                    material3,
                ));
                (objects, camera, Vector3::new(0.5, 0.7, 1.0))
            }
            Self::HDRITest => {
                let path = Path::new("tokyo.hdr");
                let image = File::open(path).unwrap();

                let bufreader = BufReader::new(image);
                let hdrdecoder = HdrDecoder::new(bufreader).unwrap();
                let im_width = hdrdecoder.metadata().width;
                let im_height = hdrdecoder.metadata().height;

                let image_v = hdrdecoder.read_image_hdr().unwrap();
                /*let mut max=0.0;
                let mut index = 0;
                image_v.iter().enumerate().for_each(|(i,pixel)|{
                    let acc = pixel[0]+pixel[1]+pixel[2];
                    if max<acc{
                        max=acc;
                    index = i;}
                    }
                );
                dbg!(max, &image_v[index]);*/
                let look_from = Vector3::new(-6.0, 1.0, 0.0);
                let look_at = Vector3::new(0.0, 0.0, 0.0);
                let vup = Vector3::new(0.0, 1.0, 0.0);
                let dist_to_focus = (look_at - look_from).magnitude();
                let aperture = 0.1;

                let camera = Camera::new(
                    look_from,
                    look_at,
                    vup,
                    40.0,
                    width / height,
                    aperture,
                    dist_to_focus,
                );
                let hdri = Material::Hdri {
                    texture: Texture::Hdri {
                        image_v: Arc::new(image_v),
                        width: im_width as f64,
                        height: im_height as f64,
                    },
                };
                let cr = Material::Dielectric {
                    index_of_refraction: 1.5,
                };
                let metal = Material::Metal {
                    albedo: Vector3::new(1.0, 0.86, 0.57),
                    fuzz: 0.0,
                };
                let path = Path::new("marble4.jpg");
                let image = image::open(path)
                    .map_err(|e| format!("Failed to read image from {:?}: {}", path, e))
                    .unwrap();
                let image_v = image.as_bytes();
                let material_ground = Material::TexturedLambertian {
                    texture: Texture::Image {
                        image_v: Arc::new(image_v.to_vec()),
                        width: image.width() as f64,
                        height: image.height() as f64,
                    },
                };

                let objects = vec![
                    Object::build_sphere(Vector3::new(0.0, 0.0, 0.0), 15.0, hdri),
                    Object::build_sphere(Vector3::new(0.0, 0.0, -1.0), 0.98, cr),
                    Object::build_sphere(Vector3::new(0.0, 0.0, 1.0), 0.98, metal),
                    Object::build_xz_rect(-5.0, 5.0, -5.0, 5.0, -0.98, material_ground, false),
                ];

                (objects, camera, Vector3::new(0.5, 0.7, 1.0))
            },

            Self::RectangleLight => {
                let path = Path::new("marble.jpg");
                let image = image::open(path)
                    .map_err(|e| format!("Failed to read image from {:?}: {}", path, e))
                    .unwrap();
                let image_v = image.as_bytes();

                let look_from = Vector3::new(26.0, 3.0, 6.0);
                let look_at = Vector3::new(0.0, 2.0, 0.0);
                let vup = Vector3::new(0.0, 1.0, 0.0);
                let dist_to_focus = (look_at - look_from).magnitude();
                let aperture = 0.1;

                let camera = Camera::new(
                    look_from,
                    look_at,
                    vup,
                    20.0,
                    width / height,
                    aperture,
                    dist_to_focus,
                );
                let marble_material = Material::TexturedLambertian {
                    texture: Texture::Image {
                        image_v: Arc::new(image_v.to_vec()),
                        width: image.width() as f64,
                        height: image.height() as f64,
                    },
                };

                let mut objects = vec![Object::build_sphere(
                    Vector3::new(0.0, 2.0, 0.0),
                    1.99,
                    marble_material,
                )];
                let material_ground = Material::Lambertian {
                    albedo: Vector3::new(0.65, 0.65, 0.5),
                };
                objects.push(Object::build_sphere(
                    Vector3::new(0.0, -1000.0, 0.0),
                    1000.0,
                    material_ground,
                ));

                let diffsphere = Material::DiffuseLight {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(0.2, 0.8, 0.6) * 1.5,
                    },
                };
                objects.push(Object::build_sphere(
                    Vector3::new(0.5, 2.0, 4.0),
                    2.0,
                    diffsphere,
                ));

                let crystal = Material::Dielectric {
                    index_of_refraction: 1.5,
                };
                objects.push(Object::build_sphere(
                    Vector3::new(3.2, 1.0, 1.9),
                    1.0,
                    crystal.clone(),
                ));
                objects.push(Object::build_sphere(
                    Vector3::new(0.0, 2.0, 0.0),
                    2.0,
                    crystal,
                ));

                let difflight = Material::DiffuseLight {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(4.0, 4.0, 4.0),
                    },
                };
                objects.push(Object::build_xy_rect(
                    -80.0,
                    100.0,
                    -10.0,
                    100.0,
                    -2.0,
                    Material::Lambertian {
                        albedo: Vector3::new(0.65, 0.65, 0.5),
                    },
                    false,
                ));
                objects.push(Object::build_xy_rect(
                    3.0, 5.0, 1.0, 3.0, -1.99, difflight, false,
                ));

                (objects, camera, Vector3::new(0.1, 0.2, 0.4))
            }

            Self::CornellBox => {
                let look_from = Vector3::new(278.0, 278.0, -800.0);
                let look_at = Vector3::new(278.0, 278.0, 0.0);
                let vup = Vector3::new(0.0, 1.0, 0.0);
                let dist_to_focus = 10.0;
                let aperture = 0.0;

                let camera = Camera::new(
                    look_from,
                    look_at,
                    vup,
                    40.0,
                    width / height,
                    aperture,
                    dist_to_focus,
                );

                let red = Material::Lambertian {
                    albedo: Vector3::new(0.65, 0.05, 0.05),
                };
                let white = Material::Lambertian {
                    albedo: Vector3::new(0.73, 0.73, 0.73),
                };
                let green = Material::Lambertian {
                    albedo: Vector3::new(0.12, 0.45, 0.15),
                };

                let difflight = Material::DiffuseLight {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(15.0, 15.0, 15.0),
                    },
                };

                let mut objects = vec![Object::build_yz_rect(
                    0.0, 555.0, 0.0, 555.0, 555.0, green, false,
                )];
                objects.push(Object::build_yz_rect(
                    0.0, 555.0, 0.0, 555.0, 0.0, red, false,
                ));
                objects.push(Object::build_xz_rect(
                    213.0, 343.0, 227.0, 332.0, 554.0, difflight, false,
                ));
                objects.push(Object::build_xz_rect(
                    0.0,
                    555.0,
                    0.0,
                    555.0,
                    0.0,
                    white.clone(),
                    false,
                ));
                objects.push(Object::build_xz_rect(
                    0.0,
                    555.0,
                    0.0,
                    555.0,
                    555.0,
                    white.clone(),
                    false,
                ));
                objects.push(Object::build_xy_rect(
                    0.0,
                    555.0,
                    0.0,
                    555.0,
                    555.0,
                    white.clone(),
                    false,
                ));

                let box1 = 
                    Object::build_prism(
                        Vector3::new(0.0, 0.0, 0.0),
                        Vector3::new(165.0, 330.0, 165.0),
                        white.clone(),
                    )
                    .rotate_y(15.0)
                    .translate(Vector3::new(265.0, 0.0, 295.0));
               
                  let box2 =   Object::build_prism(
                        Vector3::new(0.0, 0.0, 0.0),
                        Vector3::new(165.0, 165.0, 165.0),
                        white.clone(),
                    )
                    .rotate_y(-18.0)
                    .translate(Vector3::new(130.0, 0.0, 65.0));

                    objects.push(box1);
                    objects.push(box2);

                    (objects,camera, Vector3::new(0.0,0.0,0.0))
            },
            Scenes::Volumes=>{
                let look_from = Vector3::new(478.0, 278.0, -600.0);
                let look_at = Vector3::new(278.0, 278.0, 0.0);
                let vup = Vector3::new(0.0, 1.0, 0.0);
                let dist_to_focus = 10.0;
                let aperture = 0.0;
                
                let camera = Camera::new(
                    look_from,
                    look_at,
                    vup,
                    40.0,
                    width / height,
                    aperture,
                    dist_to_focus,
                );
                
                let ground = Material::Lambertian {
                    albedo: Vector3::new(0.48, 0.83, 0.53),
                };
                let mut boxes = vec![];
                let boxes_per_side = 20;
                
                for i in 0..boxes_per_side{
                    for j in 0..boxes_per_side{
                        let w = 100.0;
                        let x0 = -1000.0+i as f64*w;
                        let z0 = -1000.0 + j as f64*w;
                        let y0 = 0.0;
                        let x1 = x0+w;
                        let y1 = rng.gen_range(1.0..101.0);
                        let z1 = z0+w;
                
                        boxes.push(Object::build_prism(Vector3::new(x0,y0,z0), Vector3::new(x1,y1,z1), ground.clone()));
                    }
                }
                let mut objects = vec![BVHNode::from(&mut boxes)];
                
                let light = Material::DiffuseLight {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(7.0, 7.0, 7.0),
                    },
                };
                
                objects.push(Object::build_xz_rect(
                    123.0, 423.0, 147.0, 412.0, 554.0, light, false,
                ));
                let center1 = Vector3::new(400.0,400.0,200.0);
                
                let moving_sphere_material = Material::Lambertian { albedo: Vector3::new(0.7,0.3,0.1) };
                objects.push(Object::build_sphere(center1,50.0, moving_sphere_material));
                objects.push(Object::build_sphere(Vector3::new(260.0,150.0,45.0),50.0, Material::Dielectric{index_of_refraction:1.5}));
                
                objects.push(Object::build_sphere(Vector3::new(0.0,150.0,145.0),50.0, Material::Metal{albedo: Vector3::new(0.8,0.8,0.9), fuzz:1.0}));

                let glossy = Object::build_sphere(Vector3::new(360.0,150.0,145.0),70.0, Material::Dielectric{index_of_refraction:1.5});
                objects.push(glossy.clone());
                objects.push(Object::build_constant_medium(glossy, 0.2,Vector3::new(0.2,0.4,0.9)));

                
                let boundary = Object::build_sphere(Vector3::new(0.0,0.0,0.0),5000.0,Material::Dielectric{index_of_refraction:1.5});
                objects.push(Object::build_constant_medium(boundary, 0.0001, Vector3::new(1.0,1.0,1.0)));
                
                let path = Path::new("marble.jpg");
                let image = image::open(path)
                    .map_err(|e| format!("Failed to read image from {:?}: {}", path, e))
                    .unwrap();
                let image_v = image.as_bytes();
                let emat = Material::TexturedLambertian { texture: Texture::Image { image_v: Arc::new(image_v.to_vec()), width: image.width() as f64, height: image.height() as f64} };
                objects.push(Object::build_sphere(Vector3::new(280.0,240.0,400.0), 100.0,emat));
                
                
                let mut box_of_balls = vec![];
                
                let white = Material::Lambertian { albedo: Vector3::new(0.73,0.73,0.73) };
                let ns = 1000;
                for _j in 0..ns{
                    box_of_balls.push(Object::build_sphere(Vector3::random_vec(0.0,165.0, &mut rng), 10.0, white.clone()));
                }
                objects.push(BVHNode::from(&mut box_of_balls).rotate_y(15.0).translate(Vector3::new(-100.0,270.0,395.0)));
                
                (objects, camera, Vector3::new(0.0, 0.0, 0.0))
            }
        }
    }  
}
