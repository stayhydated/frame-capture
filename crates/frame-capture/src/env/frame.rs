use super::*;

impl CaptureFrame {
    /// Creates a positive capture frame number.
    ///
    /// # Panics
    ///
    /// Panics when `value` is zero. Use [`Self::try_new`] when zero is
    /// recoverable input.
    pub const fn new(value: u32) -> Self {
        match Self::try_new(value) {
            Some(frame) => frame,
            None => panic!("capture frame must be greater than zero"),
        }
    }

    pub const fn from_nonzero(value: NonZeroU32) -> Self {
        Self(value)
    }

    pub const fn try_new(value: u32) -> Option<Self> {
        let Some(value) = NonZeroU32::new(value) else {
            return None;
        };

        Some(Self(value))
    }

    pub const fn get(self) -> u32 {
        self.0.get()
    }

    pub const fn into_nonzero(self) -> NonZeroU32 {
        self.0
    }
}

impl fmt::Display for CaptureFrame {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl From<CaptureFrame> for NonZeroU32 {
    fn from(value: CaptureFrame) -> Self {
        value.into_nonzero()
    }
}

impl From<NonZeroU32> for CaptureFrame {
    fn from(value: NonZeroU32) -> Self {
        Self::from_nonzero(value)
    }
}

impl FromStr for CaptureFrame {
    type Err = ParseCaptureFrameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let frame = value
            .parse::<u32>()
            .map_err(|_| ParseCaptureFrameError::Invalid {
                value: value.to_owned(),
            })?;

        Self::try_new(frame).ok_or(ParseCaptureFrameError::Zero)
    }
}

impl Serialize for CaptureFrame {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.get())
    }
}

impl<'de> Deserialize<'de> for CaptureFrame {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Self::try_new(value).ok_or_else(|| de::Error::custom(ParseCaptureFrameError::Zero))
    }
}
