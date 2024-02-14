type Hz = f32;
type KHz = f32;

impl Default for Hz {
    fn default() -> Self {
        432.0
    }
}

impl Hz {
    pub fn new(hz: Hz) -> Self {
        Hz { hz }
    }

    pub fn set(&mut self, hz: Hz) {
        self.hz = hz;
    }

    pub fn to_khz(&self) -> KHz {
        self.hz / 1000.0
    }

    pub fn add(&mut self, hz: Hz) {
        self.hz += hz;
    }

    pub fn sub(&mut self, hz: Hz) {
        self.hz -= hz;
    }

    pub fn mul(&mut self, hz: Hz) {
        self.hz *= hz;
    }

    pub fn div(&mut self, hz: Hz) {
        self.hz /= hz;
    }

    pub fn rem(&mut self, hz: Hz) {
        self.hz %= hz;
    }

    pub fn neg(&mut self) {
        self.hz = -self.hz;
    }

    pub fn inc(&mut self) {
        self.hz += 1.0;
    }

    pub fn dec(&mut self) {
        self.hz -= 1.0;
    }

    pub fn add_khz(&mut self, khz: KHz) {
        self.hz += khz * 1000.0;
    }

    pub fn sub_khz(&mut self, khz: KHz) {
        self.hz -= khz * 1000.0;
    }

    pub fn mul_khz(&mut self, khz: KHz) {
        self.hz *= khz * 1000.0;
    }

    pub fn div_khz(&mut self, khz: KHz) {
        self.hz /= khz * 1000.0;
    }

    pub fn rem_khz(&mut self, khz: KHz) {
        self.hz %= khz * 1000.0;
    }

    pub fn neg_khz(&mut self) {
        self.hz = -self.hz;
    }

    pub fn inc_khz(&mut self) {
        self.hz += 1.0 * 1000.0;
    }

    pub fn dec_khz(&mut self) {
        self.hz -= 1.0 * 1000.0;
    }

    pub fn set_khz(&mut self, khz: KHz) {
        self.hz = khz * 1000.0;
    }
}

impl Default for KHz {
    fn default() -> Self {
        0.432
    }
}

impl KHz {
    pub fn new(khz: KHz) -> Self {
        KHz { khz }
    }

    pub fn add(&mut self, khz: KHz) {
        self.khz += khz;
    }

    pub fn sub(&mut self, khz: KHz) {
        self.khz -= khz;
    }

    pub fn mul(&mut self, khz: KHz) {
        self.khz *= khz;
    }

    pub fn div(&mut self, khz: KHz) {
        self.khz /= khz;
    }

    pub fn rem(&mut self, khz: KHz) {
        self.khz %= khz;
    }

    pub fn neg(&mut self) {
        self.khz = -self.khz;
    }

    pub fn inc(&mut self) {
        self.khz += 1.0;
    }

    pub fn dec(&mut self) {
        self.khz -= 1.0;
    }

    pub fn add_hz(&mut self, hz: Hz) {
        self.khz += hz / 1000.0;
    }

    pub fn sub_hz(&mut self, hz: Hz) {
        self.khz -= hz / 1000.0;
    }

    pub fn mul_hz(&mut self, hz: Hz) {
        self.khz *= hz / 1000.0;
    }

    pub fn div_hz(&mut self, hz: Hz) {
        self.khz /= hz / 1000.0;
    }

    pub fn rem_hz(&mut self, hz: Hz) {
        self.khz %= hz / 1000.0;
    }

    pub fn neg_hz(&mut self) {
        self.khz = -self.khz;
    }

    pub fn inc_hz(&mut self) {
        self.khz += 1.0 / 1000.0;
    }

    pub fn dec_hz(&mut self) {
        self.khz -= 1.0 / 1000.0;
    }

    pub fn set(&mut self, khz: KHz) {
        self.khz = khz;
    }

    pub fn to_hz(&self) -> Hz {
        self.khz * 1000.0
    }

    pub fn set_hz(&mut self, hz: Hz) {
        self.khz = hz / 1000.0;
    }
}
