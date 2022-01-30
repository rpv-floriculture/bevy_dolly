use bevy::prelude::Mut;

pub trait Transform2Bevy {
    fn transform_2_bevy(&mut self, transform: dolly::transform::Transform);
}

impl Transform2Bevy for Mut<'_, bevy::prelude::Transform> {
    fn transform_2_bevy(&mut self, transform: dolly::transform::Transform) {
        let (translation, rotation) = transform.into_position_rotation();
        self.translation = bevy::math::Vec3::new(translation.x, translation.y, translation.z);
        self.rotation = bevy::math::Quat::from_xyzw(rotation.x, rotation.y, rotation.z, rotation.w);
    }
}

pub trait Transform2DollyMut {
    fn transform_2_dolly_mut(&self) -> dolly::transform::Transform;
}

impl Transform2DollyMut for Mut<'_, bevy::prelude::Transform> {
    fn transform_2_dolly_mut(&self) -> dolly::transform::Transform {
        let t = self.translation;
        let r = self.rotation;
        dolly::transform::Transform {
            position: dolly::glam::Vec3::new(t.x, t.y, t.z),
            rotation: dolly::glam::Quat::from_xyzw(r.x, r.y, r.z, r.w),
        }
    }
}

pub trait Transform2Dolly {
    fn transform_2_dolly(&self) -> dolly::transform::Transform;
}

impl Transform2Dolly for bevy::prelude::Transform {
    fn transform_2_dolly(&self) -> dolly::transform::Transform {
        let t = self.translation;
        let r = self.rotation;
        dolly::transform::Transform {
            position: dolly::glam::Vec3::new(t.x, t.y, t.z),
            rotation: dolly::glam::Quat::from_xyzw(r.x, r.y, r.z, r.w),
        }
    }
}
