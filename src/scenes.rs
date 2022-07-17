use std::{path::Path, sync::Arc};

use crate::{
    background::{load_hdri, Background},
    camera::Camera,
    material::Material,
    object::Object,
    rectangle::Prism,
    texture::Texture,
    triangle_mesh::TriangleMesh,
    utilities::vector3::Vector3,
};
use rand::Rng;
#[allow(dead_code)]
pub enum Scenes {
    Basic,
    BasicChecker,
    HDRITest,
    HDRISun,
    RectangleLight,
    CornellBox,
    Volumes,
    Balls,
    Model3D,
    David,
}

impl Scenes {
    pub fn get(&self, width: f32, height: f32) -> SceneConfig {
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
                    3.0,
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
                        let x = a as f32;
                        let y = b as f32;
                        let choose_material = rng.gen_range(1..100);
                        let center = Vector3::new(
                            x + 0.9 * rng.gen::<f32>(),
                            0.2,
                            y + 0.9 * rng.gen::<f32>(),
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
                SceneConfig::new(
                    objects,
                    camera,
                    vec![],
                    Background::new_plain(Vector3::new(0.5, 0.7, 1.0)),
                )
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
                    3.0,
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
                        let x = a as f32;
                        let y = b as f32;
                        let choose_material = rng.gen_range(1..100);
                        let center = Vector3::new(
                            x + 0.9 * rng.gen::<f32>(),
                            0.2,
                            y + 0.9 * rng.gen::<f32>(),
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
                SceneConfig::new(
                    objects,
                    camera,
                    vec![],
                    Background::new_plain(Vector3::new(0.5, 0.7, 1.0)),
                )
            }
            Self::HDRITest => {
                let (env_map, hdri) = load_hdri("HDRIs/tokyo.hdr", 0.0);

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
                    2.0,
                );

                let cr = Material::Dielectric {
                    index_of_refraction: 1.5,
                };
                let metal = Material::Metal {
                    albedo: Vector3::new(1.0, 0.86, 0.57),
                    fuzz: 0.0,
                };
                let path = Path::new("textures/marble4.jpg");
                let image = image::open(path)
                    .map_err(|e| format!("Failed to read image from {:?}: {}", path, e))
                    .unwrap();
                let image_v = image.as_bytes();
                let material_ground = Material::TexturedLambertian {
                    texture: Texture::Image {
                        image_v: Arc::new(image_v.to_vec()),
                        width: image.width() as f32,
                        height: image.height() as f32,
                    },
                };

                let objects = vec![
                    Object::build_sphere(Vector3::new(0.0, 0.0, -1.0), 0.98, cr.clone()),
                    Object::build_sphere(Vector3::new(0.0, 0.0, 1.0), 0.98, metal),
                    Object::build_xz_rect(-5.0, 5.0, -5.0, 5.0, -0.98, material_ground, false),
                ];

                SceneConfig::new(objects, camera, vec![env_map], Background::new_hdri(hdri))
            }

            Self::HDRISun => {
                let (env_map, hdri) = load_hdri("HDRIs/studio.hdr", 120.0);

                let look_from = Vector3::new(-6.0, 0.8, 0.0);
                let look_at = Vector3::new(0.0, 0.0, 0.0);
                let vup = Vector3::new(0.0, 1.0, 0.0);
                let dist_to_focus = (look_at - look_from).magnitude();
                let aperture = 0.0;

                let camera = Camera::new(
                    look_from,
                    look_at,
                    vup,
                    40.0,
                    width / height,
                    aperture,
                    dist_to_focus,
                    0.8,
                );

                let path = Path::new("textures/marble4.jpg");
                let image = image::open(path)
                    .map_err(|e| format!("Failed to read image from {:?}: {}", path, e))
                    .unwrap();
                let image_v = image.as_bytes();
                let material_ground = Material::TexturedLambertian {
                    texture: Texture::Image {
                        image_v: Arc::new(image_v.to_vec()),
                        width: image.width() as f32,
                        height: image.height() as f32,
                    },
                };
                let mut teapot = TriangleMesh::load(
                    "objs/teapot.obj",
                    0.3,
                    Vector3::new(-2.1, -0.98, 0.0),
                    60.0,
                    1,
                    /*Material::BlinnPhong {
                        color: Vector3::new(0.12, 0.45, 0.15),
                        k_specular: 0.08,
                        exponent: 50.0,
                    },*/
                    Material::FresnelBlend {
                        r_d: Vector3::new(0.0, 0.0, 0.0),
                        r_s: Vector3::new(0.983, 0.991, 0.995),
                        k_specular: 1.0,
                        nu: 100.0,
                        nv: 100.0,
                    },
                );

                let mut bunny = TriangleMesh::load(
                    "objs/stanford-bunny.obj",
                    13.0,
                    Vector3::new(0.0, -1.42, -1.3),
                    -70.0,
                    1,
                    /*Material::BlinnPhong {
                        color: Vector3::new(0.6, 0.2, 0.1),
                        k_specular: 0.1,
                        exponent: 100.0,
                    },*/
                         Material::FresnelBlend {
                        r_d: Vector3::new(0.6, 0.2, 0.1),
                        r_s: Vector3::new(1.0,1.0,1.0),
                        k_specular: 0.1,
                        nu: 100.0,
                        nv: 100.0,
                    },
                );

                let mut objects = vec![
                    /*Object::build_sphere(
                        Vector3::new(0.0, 0.0, -0.98),
                        0.98,
                        Material::BlinnPhong {
                            color: Vector3::new(0.6, 0.2, 0.2),
                            k_specular: 0.1,
                            exponent: 150.0,
                        },
                    ),*/
                    Object::build_sphere(
                        Vector3::new(0.0, 0.0, 0.98),
                        0.98,
                        Material::Metal {
                            albedo: Vector3::new(0.542, 0.497, 0.449),
                            fuzz: 0.0,
                        },
                       
                    ),
                    Object::build_xz_rect(-5.0, 5.0, -5.0, 5.0, -0.98, material_ground, false),
                ];

                teapot.push_to_objects(&mut objects);
                //bunny.push_to_objects(&mut objects);

                SceneConfig::new(objects, camera, vec![env_map], Background::new_hdri(hdri))
            }

            Self::RectangleLight => {
                let path = Path::new("textures/marble.jpg");
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
                    2.0,
                );
                let marble_material = Material::TexturedLambertian {
                    texture: Texture::Image {
                        image_v: Arc::new(image_v.to_vec()),
                        width: image.width() as f32,
                        height: image.height() as f32,
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
                let sphere_blue_light =
                    Object::build_sphere(Vector3::new(0.5, 2.0, 4.0), 2.0, diffsphere);
                objects.push(sphere_blue_light.clone());

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

                let rectangle_light =
                    Object::build_xy_rect(3.0, 5.0, 1.0, 3.0, -1.99, difflight.clone(), false);

                objects.push(rectangle_light.clone());

                SceneConfig::new(
                    objects,
                    camera,
                    vec![
                        rectangle_light,
                        sphere_blue_light,
                        Object::build_sphere(Vector3::new(3.2, 1.0, 1.9), 1.0, Material::default()),
                    ],
                    Background::new_plain(Vector3::new(0.1, 0.2, 0.4)),
                )
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
                    3.0,
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
                        albedo: Vector3::new(25.0, 25.0, 25.0),
                    },
                };

                let mut objects = vec![Object::build_yz_rect(
                    0.0, 555.0, 0.0, 555.0, 555.0, green, false,
                )];
                objects.push(Object::build_yz_rect(
                    0.0, 555.0, 0.0, 555.0, 0.0, red, false,
                ));
                let rect_light =
                    Object::build_xz_rect(213.0, 343.0, 227.0, 332.0, 554.0, difflight, true);
                objects.push(rect_light.clone());
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
                /*let aluminum = Material::Metal {
                    albedo: Vector3::new(0.8, 0.85, 0.88),
                    fuzz: 0.0,
                };*/

                let mut box1 = Prism::build_prism(
                    Vector3::new(0.0, 0.0, 0.0),
                    Vector3::new(165.0, 330.0, 165.0),
                    white.clone(),
                )
                .rotate_y(15.0)
                .translate(Vector3::new(265.0, 0.0, 295.0));

                /*let box2 = Prism::build_prism(
                    Vector3::new(0.0, 0.0, 0.0),
                    Vector3::new(165.0, 165.0, 165.0),
                    white.clone(),
                )
                .rotate_y(-18.0)
                .translate(Vector3::new(130.0, 0.0, 65.0));*/

                let sphere = Object::build_sphere(
                    Vector3::new(190.0, 90.0, 190.0),
                    90.0,
                    Material::Dielectric {
                        index_of_refraction: 1.5,
                    },
                );
                box1.push_to_objects(&mut objects);
                objects.push(sphere);

                let (env_map, hdri) = load_hdri("HDRIs/sun.hdr", -15.0);

                SceneConfig::new(
                    objects,
                    camera,
                    vec![
                        rect_light,
                        Object::build_sphere(
                            Vector3::new(190.0, 90.0, 190.0),
                            90.0,
                            Material::default(),
                        ),
                    ],
                    Background::new_plain(Vector3::new(0.0, 0.0, 0.0)),
                    // Background::new_hdri(hdri)
                )
            }
            Scenes::Volumes => {
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
                    2.0,
                );

                let ground = Material::Lambertian {
                    albedo: Vector3::new(0.48, 0.83, 0.53),
                };
                let mut objects = vec![];
                let boxes_per_side = 20;

                for i in 0..boxes_per_side {
                    for j in 0..boxes_per_side {
                        let w = 100.0;
                        let x0 = -1000.0 + i as f32 * w;
                        let z0 = -1000.0 + j as f32 * w;
                        let y0 = 0.0;
                        let x1 = x0 + w;
                        let y1 = rng.gen_range(1.0..101.0);
                        let z1 = z0 + w;

                        let mut prism = Prism::build_prism(
                            Vector3::new(x0, y0, z0),
                            Vector3::new(x1, y1, z1),
                            ground.clone(),
                        );

                        prism.push_to_objects(&mut objects);
                    }
                }

                let light = Material::DiffuseLight {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(7.0, 7.0, 7.0),
                    },
                };

                objects.push(Object::build_xz_rect(
                    123.0, 423.0, 147.0, 412.0, 554.0, light, true,
                ));
                let center1 = Vector3::new(400.0, 400.0, 200.0);

                let moving_sphere_material = Material::Lambertian {
                    albedo: Vector3::new(0.7, 0.3, 0.1),
                };
                objects.push(Object::build_sphere(center1, 50.0, moving_sphere_material));
                objects.push(Object::build_sphere(
                    Vector3::new(260.0, 150.0, 45.0),
                    50.0,
                    Material::Dielectric {
                        index_of_refraction: 1.5,
                    },
                ));

                objects.push(Object::build_sphere(
                    Vector3::new(0.0, 150.0, 145.0),
                    50.0,
                    Material::Metal {
                        albedo: Vector3::new(0.8, 0.8, 0.9),
                        fuzz: 1.0,
                    },
                ));

                let glossy = Object::build_sphere(
                    Vector3::new(360.0, 150.0, 145.0),
                    70.0,
                    Material::Dielectric {
                        index_of_refraction: 1.5,
                    },
                );
                objects.push(glossy.clone());
                objects.push(Object::build_constant_medium(
                    glossy,
                    0.2,
                    Vector3::new(0.2, 0.4, 0.9),
                ));

                let boundary =
                    Object::build_sphere(Vector3::new(0.0, 0.0, 0.0), 5000.0, Material::default());
                objects.push(Object::build_constant_medium(
                    boundary,
                    0.0001,
                    Vector3::new(1.0, 1.0, 1.0),
                ));

                let path = Path::new("textures/earthmap.jpg");
                let image = image::open(path)
                    .map_err(|e| format!("Failed to read image from {:?}: {}", path, e))
                    .unwrap();
                let image_v = image.as_bytes();
                let emat = Material::TexturedLambertian {
                    texture: Texture::Image {
                        image_v: Arc::new(image_v.to_vec()),
                        width: image.width() as f32,
                        height: image.height() as f32,
                    },
                };
                objects.push(Object::build_sphere(
                    Vector3::new(280.0, 240.0, 400.0),
                    100.0,
                    emat,
                ));

                let white = Material::Lambertian {
                    albedo: Vector3::new(0.73, 0.73, 0.73),
                };
                let ns = 1000;
                for _j in 0..ns {
                    objects.push(
                        Object::build_sphere(
                            Vector3::random_vec(0.0, 165.0, &mut rng),
                            10.0,
                            white.clone(),
                        )
                        .rotate_y(15.0)
                        .translate(Vector3::new(-100.0, 270.0, 395.0)),
                    );
                }

                SceneConfig::new(
                    objects,
                    camera,
                    vec![Object::build_xz_rect(
                        123.0,
                        423.0,
                        147.0,
                        412.0,
                        554.0,
                        Material::default(),
                        true,
                    )],
                    Background::new_plain(Vector3::new(0.0, 0.0, 0.0)),
                )
            }
            Self::Balls => {
                let look_from = Vector3::new(378.0, 178.0, -640.0);
                let look_at = Vector3::new(320.0, 133.0, 60.0);
                let vup = Vector3::new(0.0, 1.0, 0.0);
                let dist_to_focus = 700.0;
                let aperture = 3.5;

                let camera = Camera::new(
                    look_from,
                    look_at,
                    vup,
                    40.0,
                    width / height,
                    aperture,
                    dist_to_focus,
                    1.0,
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
                        albedo: Vector3::new(4.50, 4.50, 4.50),
                    },
                };

                let mut objects = vec![Object::build_yz_rect(
                    0.0,
                    355.0,
                    -400.0,
                    755.0,
                    755.0,
                    Material::BlinnPhong {
                        color: Vector3::new(0.12, 0.45, 0.15),
                        k_specular: 0.1,
                        exponent: 1500.0,
                    },
                    false,
                )];
                objects.push(Object::build_yz_rect(
                    0.0,
                    355.0,
                    -400.0,
                    755.0,
                    0.0,
                    Material::BlinnPhong {
                        color: Vector3::new(0.65, 0.05, 0.05),
                        k_specular: 0.1,
                        exponent: 1500.0,
                    },
                    false,
                ));
                objects.push(Object::build_xz_rect(
                    107.5, 647.5, 127.0, 372.0, 354.9, difflight, true,
                ));
                objects.push(Object::build_xz_rect(
                    0.0,
                    755.0,
                    -400.0,
                    555.0,
                    0.0,
                    Material::BlinnPhong {
                        color: Vector3::new(0.73, 0.73, 0.73),
                        k_specular: 0.1,
                        exponent: 1500.0,
                    },
                    false,
                ));
                objects.push(Object::build_xz_rect(
                    0.0,
                    755.0,
                    -400.0,
                    555.0,
                    355.0,
                    Material::BlinnPhong {
                        color: Vector3::new(0.73, 0.73, 0.73),
                        k_specular: 0.1,
                        exponent: 1500.0,
                    },
                    false,
                ));
                objects.push(Object::build_xy_rect(
                    0.0,
                    755.0,
                    0.0,
                    355.0,
                    555.0,
                    Material::BlinnPhong {
                        color: Vector3::new(0.73, 0.73, 0.73),
                        k_specular: 0.1,
                        exponent: 1500.0,
                    },
                    false,
                ));
                let aluminum = Material::Metal {
                    albedo: Vector3::new(0.7, 0.78, 0.85),
                    fuzz: 0.0,
                };

                for i in 0..7 {
                    let sphere = Object::build_sphere(
                        Vector3::new(68.0 + i as f32 * 104.0, 55.0, 0.0 + i as f32 * 72.0),
                        55.0,
                        Material::Blend {
                            material1: Box::new(Material::Lambertian {
                                albedo: Vector3::new(0.12, 0.15, 0.45),
                            }),
                            material2: Box::new(aluminum.clone()),
                            ratio: 1.0 - i as f32 * 1.0 / 6.0,
                        },
                    );
                    objects.push(sphere);
                }

                SceneConfig::new(
                    objects,
                    camera,
                    vec![Object::build_xz_rect(
                        107.5,
                        647.5,
                        127.0,
                        372.0,
                        354.9,
                        Material::default(),
                        true,
                    )],
                    Background::new_plain(Vector3::new(0.0, 0.0, 0.0)),
                )
            }
            Self::Model3D => {
                let look_from = Vector3::new(0.0, 0.1, 2.0);
                let look_at = Vector3::new(0.0, 0.1, 0.0);
                let vup = Vector3::new(0.0, 1.0, 0.0);
                let dist_to_focus = 2.0;
                let aperture = 0.02;

                let camera = Camera::new(
                    look_from,
                    look_at,
                    vup,
                    10.0,
                    width / height,
                    aperture,
                    dist_to_focus,
                    4.0,
                );

                let difflight = Material::DiffuseLight {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(1.80, 1.80, 1.80),
                    },
                };
                let gold = Material::Metal {
                    albedo: Vector3::new(1.0, 0.86, 0.57),
                    fuzz: 0.5,
                };

                let skull = TriangleMesh::load(
                    "objs/skull.obj",
                    0.04,
                    Vector3::new(-0.1, 0.0, 0.0),
                    15.0,
                    1,
                    Material::Dielectric {
                        index_of_refraction: 1.5,
                    },
                );
                let mut objects = skull.triangles;
                //vec![skull.rotate_y(15.0).translate(Vector3::new(-0.1, 0.1, 0.0))];
                objects.push(Object::build_xz_rect(
                    -2.0,
                    2.0,
                    -2.0,
                    2.0,
                    0.0,
                    Material::default(),
                    true,
                ));
                let mut dragon = TriangleMesh::load(
                    "objs/dragon.obj",
                    0.013,
                    Vector3::new(0.1, 0.0, 0.0),
                    0.0,
                    0,
                    gold,
                );
                dragon.push_to_objects(&mut objects);

                objects.push(Object::build_xz_rect(
                    -0.5, 0.5, -0.5, 0.5, 0.6, difflight, true,
                ));
                objects.push(Object::build_xy_rect(
                    -0.5,
                    0.5,
                    -2.0,
                    2.5,
                    -1.0,
                    Material::Lambertian {
                        albedo: Vector3::new(0.1, 0.15, 0.25),
                    },
                    true,
                ));
                let diffsphere = Material::DiffuseLight {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(0.2, 0.8, 0.6) * 1.5,
                    },
                };
                objects.push(Object::build_sphere(
                    Vector3::new(-0.05, 0.07, -1.0 + 0.07),
                    0.07,
                    diffsphere,
                ));

                SceneConfig::new(
                    objects,
                    camera,
                    vec![
                        Object::build_xz_rect(-0.5, 0.5, -0.5, 0.5, 1.0, Material::default(), true),
                        Object::build_sphere(
                            Vector3::new(-0.05, 0.07, -1.0 + 0.07),
                            0.07,
                            Material::default(),
                        ),
                    ],
                    Background::new_plain(Vector3::new(0.0, 0.0, 0.0)),
                )
            }

            Self::David => {
                let look_from = Vector3::new(278.0, 278.0, 900.0);
                let look_at = Vector3::new(278.0, 278.0, 0.0);
                let vup = Vector3::new(0.0, 1.0, 0.0);
                let dist_to_focus = 10.0;
                let aperture = 0.0;

                let camera = Camera::new(
                    look_from,
                    look_at,
                    vup,
                    29.0,
                    width / height,
                    aperture,
                    dist_to_focus,
                    2.0,
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
                    0.0, 555.0, -555.0, 200.0, 555.0, green, false,
                )];
                objects.push(Object::build_yz_rect(
                    0.0, 555.0, -555.0, 200.0, 0.0, red, false,
                ));
                objects.push(Object::build_xz_rect(
                    213.0, 343.0, -262.0, -157.0, 554.0, difflight, true,
                ));
                objects.push(Object::build_xz_rect(
                    0.0,
                    555.0,
                    -555.0,
                    200.0,
                    0.0,
                    white.clone(),
                    false,
                ));
                objects.push(Object::build_xz_rect(
                    0.0,
                    555.0,
                    -555.0,
                    200.0,
                    555.0,
                    white.clone(),
                    false,
                ));
                objects.push(Object::build_xy_rect(
                    0.0,
                    555.0,
                    0.0,
                    555.0,
                    -555.0,
                    white.clone(),
                    false,
                ));

                let mut david = TriangleMesh::load(
                    "objs/david.obj",
                    1.2,
                    Vector3::new(135.0, 0.0, -230.0),
                    -90.0,
                    0,
                    Material::BlinnPhong {
                        color: Vector3::new(0.3, 0.7, 0.3),
                        k_specular: 0.12,
                        exponent: 150.0,
                    },
                );

                david.push_to_objects(&mut objects);

                let sphere = Object::build_sphere(
                    Vector3::new(400.0, 70.0, -190.0),
                    70.0,
                    Material::BlinnPhong {
                        color: Vector3::new(0.5, 0.3, 0.1),
                        k_specular: 0.1,
                        exponent: 50.0,
                    },
                );

                objects.push(sphere);

                SceneConfig::new(
                    objects,
                    camera,
                    vec![Object::build_xz_rect(
                        213.0,
                        343.0,
                        -262.0,
                        -157.0,
                        554.0,
                        Material::default(),
                        true,
                    )],
                    Background::new_plain(Vector3::new(0.0, 0.0, 0.0)),
                )
            } /*Self::Macintosh => {
                  let look_from = Vector3::new(27.80, 20.00, 90.00);
                  let look_at = Vector3::new(27.80, 20.00, 0.0);
                  let vup = Vector3::new(0.0, 1.0, 0.0);
                  let dist_to_focus = 10.0;
                  let aperture = 0.0;

                  let camera = Camera::new(
                      look_from,
                      look_at,
                      vup,
                      29.0,
                      width / height,
                      aperture,
                      dist_to_focus,
                  );

                  let difflight = Material::DiffuseLight {
                      texture: Texture::SolidColor {
                          albedo: Vector3::new(0.0, 25.0, 0.0),
                      },
                  };

                  let material_ground = Material::TexturedLambertian {
                      texture: Texture::Checker {
                          color1: Vector3::new(0.0, 0.0, 0.0),
                          color2: Vector3::new(249.0/255.0, 121.0/255.0, 136.0/255.0),
                      },
                  };
                  let mut objects = vec![/*Object::build_sphere(
                      Vector3::new(0.0, -10000.0, 0.0),
                      10000.0,
                      material_ground,
                  )*/];

                  let light = Object::build_xz_rect(
                      60.0, 65.0, 7.50, 12.50, 55.40, difflight, true,
                  );
                  objects.push(light.clone());

                  let helios = TriangleMesh::load(
                      "objs/helios.obj",
                      0.17,
                      Vector3::new(0.0, 0.0, 0.0),
                      -90.0,
                      0,
                      Material::Lambertian { albedo: Vector3::new(183.0/255.0,147.0/255.0,111.0/255.0) }
                      //237.0 / 255.0, 192.0 / 255.0, 151.0 / 255.0
                  );

                  helios.rotate_y(-30.0).translate(Vector3::new(23.0, 8.50, 3.0)).push_to_objects(&mut objects);

                  let sphere = Object::build_sphere(
                      Vector3::new(40.0, 7.0, 3.0),
                      7.0,
                      Material::BlinnPhong{
                          color: Vector3::new(0.5,0.3,0.1), k_specular: 0.1, exponent: 50.0,
                      },
                  );

                  objects.push(sphere);

                  SceneConfig::new(
                      objects,
                      camera,
                      vec![light],
                      Vector3::new(0.3, 15.0/255.0, 30.0/255.0),
                  )
              }*/
        }
    }
}

pub struct SceneConfig {
    pub objects: Vec<Object>,
    pub camera: Camera,
    pub light: Vec<Object>,
    pub background: Background,
}

impl SceneConfig {
    pub fn new(
        objects: Vec<Object>,
        camera: Camera,
        light: Vec<Object>,
        background: Background,
    ) -> Self {
        Self {
            objects,
            camera,
            light,
            background,
        }
    }
}
