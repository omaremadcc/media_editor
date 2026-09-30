use crate::Resolution;

impl Resolution {
    pub fn p1920_1080() -> Self {
        Self {
            width: 1920,
            height: 1080,
        }
    }

    pub fn p1280_720() -> Self {
        Self {
            width: 1280,
            height: 720,
        }
    }

    pub fn p1080_1920() -> Self {
        Self {
            width: 1080,
            height: 1920,
        }
    }

    pub fn p720_1080() -> Self {
        Self {
            width: 720,
            height: 1080,
        }
    }

    pub fn p1200_1200() -> Self {
        Self {
            width: 1200,
            height: 1200,
        }
    }
}
