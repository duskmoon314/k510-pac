#[doc = "Register `PERI_SYS_BUS_CLK_EN` reader"]
pub struct R(crate::R<PERI_SYS_BUS_CLK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERI_SYS_BUS_CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERI_SYS_BUS_CLK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERI_SYS_BUS_CLK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERI_SYS_BUS_CLK_EN` writer"]
pub struct W(crate::W<PERI_SYS_BUS_CLK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERI_SYS_BUS_CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PERI_SYS_BUS_CLK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERI_SYS_BUS_CLK_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WE_audif_pclk_en` writer - Write enable for bit 10 (audif_pclk_en)"]
pub type WE_AUDIF_PCLK_EN_W<'a> = crate::BitWriter<'a, u32, PERI_SYS_BUS_CLK_EN_SPEC, bool, 26>;
#[doc = "Fields `WE_spi_(0-3)_hclk_en` writer - Write enable for bit 6-9 (spi_\\[0-3\\]_hclk_en)"]
pub type WE_SPI__HCLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PERI_SYS_BUS_CLK_EN_SPEC, bool, O>;
#[doc = "Field `WE_i2s_2_pclk_en` writer - Write enable for bit 5 (i2s_2_pclk_en)"]
pub type WE_I2S_2_PCLK_EN_W<'a> = crate::BitWriter<'a, u32, PERI_SYS_BUS_CLK_EN_SPEC, bool, 21>;
#[doc = "Fields `WE_uart_(0-3)_pclk_en` writer - Write enable for bit 0-3 (uart_\\[0-3\\]_pclk_en)"]
pub type WE_UART__PCLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PERI_SYS_BUS_CLK_EN_SPEC, bool, O>;
#[doc = "Audio data interface slave APB port clock enable control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIF_HCLK_EN_A {
    #[doc = "0: Disable AUD_IF slave APB port clock"]
    DISABLE = 0,
    #[doc = "1: Enable AUD_IF slave APB port clock"]
    ENABLE = 1,
}
impl From<AUDIF_HCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIF_HCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `audif_hclk_en` reader - Audio data interface slave APB port clock enable control"]
pub type AUDIF_HCLK_EN_R = crate::BitReader<AUDIF_HCLK_EN_A>;
impl AUDIF_HCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIF_HCLK_EN_A {
        match self.bits {
            false => AUDIF_HCLK_EN_A::DISABLE,
            true => AUDIF_HCLK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AUDIF_HCLK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AUDIF_HCLK_EN_A::ENABLE
    }
}
#[doc = "Field `audif_hclk_en` writer - Audio data interface slave APB port clock enable control"]
pub type AUDIF_HCLK_EN_W<'a> =
    crate::BitWriter<'a, u32, PERI_SYS_BUS_CLK_EN_SPEC, AUDIF_HCLK_EN_A, 10>;
impl<'a> AUDIF_HCLK_EN_W<'a> {
    #[doc = "Disable AUD_IF slave APB port clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUDIF_HCLK_EN_A::DISABLE)
    }
    #[doc = "Enable AUD_IF slave APB port clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUDIF_HCLK_EN_A::ENABLE)
    }
}
#[doc = "SPI host \\[i\\]
slave AHB interface clock enable control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI__HCLK_EN_A {
    #[doc = "0: Disable SPI \\[i\\]
slave AHB interface clock"]
    DISABLE = 0,
    #[doc = "1: Enable SPI \\[i\\]
slave AHB interface clock"]
    ENABLE = 1,
}
impl From<SPI__HCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI__HCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `spi_(0-3)_hclk_en` reader - SPI host \\[i\\]
slave AHB interface clock enable control"]
pub type SPI__HCLK_EN_R = crate::BitReader<SPI__HCLK_EN_A>;
impl SPI__HCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI__HCLK_EN_A {
        match self.bits {
            false => SPI__HCLK_EN_A::DISABLE,
            true => SPI__HCLK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SPI__HCLK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SPI__HCLK_EN_A::ENABLE
    }
}
#[doc = "Fields `spi_(0-3)_hclk_en` writer - SPI host \\[i\\]
slave AHB interface clock enable control"]
pub type SPI__HCLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PERI_SYS_BUS_CLK_EN_SPEC, SPI__HCLK_EN_A, O>;
impl<'a, const O: u8> SPI__HCLK_EN_W<'a, O> {
    #[doc = "Disable SPI \\[i\\]
slave AHB interface clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SPI__HCLK_EN_A::DISABLE)
    }
    #[doc = "Enable SPI \\[i\\]
slave AHB interface clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SPI__HCLK_EN_A::ENABLE)
    }
}
#[doc = "I2S 2 slave APB interface clock enable control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_2_PCLK_EN_A {
    #[doc = "0: Disable I2S 2 slave APB port clock"]
    DISABLE = 0,
    #[doc = "1: Enable I2S 2 slave APB port clock"]
    ENABLE = 1,
}
impl From<I2S_2_PCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_2_PCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `i2s_2_pclk_en` reader - I2S 2 slave APB interface clock enable control"]
pub type I2S_2_PCLK_EN_R = crate::BitReader<I2S_2_PCLK_EN_A>;
impl I2S_2_PCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_2_PCLK_EN_A {
        match self.bits {
            false => I2S_2_PCLK_EN_A::DISABLE,
            true => I2S_2_PCLK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == I2S_2_PCLK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == I2S_2_PCLK_EN_A::ENABLE
    }
}
#[doc = "Field `i2s_2_pclk_en` writer - I2S 2 slave APB interface clock enable control"]
pub type I2S_2_PCLK_EN_W<'a> =
    crate::BitWriter<'a, u32, PERI_SYS_BUS_CLK_EN_SPEC, I2S_2_PCLK_EN_A, 5>;
impl<'a> I2S_2_PCLK_EN_W<'a> {
    #[doc = "Disable I2S 2 slave APB port clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2S_2_PCLK_EN_A::DISABLE)
    }
    #[doc = "Enable I2S 2 slave APB port clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2S_2_PCLK_EN_A::ENABLE)
    }
}
#[doc = "UART host \\[i\\]
slave APB interface clock enable control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART__PCLK_EN_A {
    #[doc = "0: Disable UART \\[i\\]
slave APB port clock"]
    DISABLE = 0,
    #[doc = "1: Enable UART \\[i\\]
slave APB port clock"]
    ENABLE = 1,
}
impl From<UART__PCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: UART__PCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `uart_(0-3)_pclk_en` reader - UART host \\[i\\]
slave APB interface clock enable control"]
pub type UART__PCLK_EN_R = crate::BitReader<UART__PCLK_EN_A>;
impl UART__PCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART__PCLK_EN_A {
        match self.bits {
            false => UART__PCLK_EN_A::DISABLE,
            true => UART__PCLK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UART__PCLK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UART__PCLK_EN_A::ENABLE
    }
}
#[doc = "Fields `uart_(0-3)_pclk_en` writer - UART host \\[i\\]
slave APB interface clock enable control"]
pub type UART__PCLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PERI_SYS_BUS_CLK_EN_SPEC, UART__PCLK_EN_A, O>;
impl<'a, const O: u8> UART__PCLK_EN_W<'a, O> {
    #[doc = "Disable UART \\[i\\]
slave APB port clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UART__PCLK_EN_A::DISABLE)
    }
    #[doc = "Enable UART \\[i\\]
slave APB port clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UART__PCLK_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 10 - Audio data interface slave APB port clock enable control"]
    #[inline(always)]
    pub fn audif_hclk_en(&self) -> AUDIF_HCLK_EN_R {
        AUDIF_HCLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "SPI host \\[i\\]
slave AHB interface clock enable control"]
    #[inline(always)]
    pub unsafe fn spi__hclk_en(&self, n: u8) -> SPI__HCLK_EN_R {
        SPI__HCLK_EN_R::new(((self.bits >> (n + 6)) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI host \\[i\\]
slave AHB interface clock enable control"]
    #[inline(always)]
    pub fn spi_0_hclk_en(&self) -> SPI__HCLK_EN_R {
        SPI__HCLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI host \\[i\\]
slave AHB interface clock enable control"]
    #[inline(always)]
    pub fn spi_1_hclk_en(&self) -> SPI__HCLK_EN_R {
        SPI__HCLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI host \\[i\\]
slave AHB interface clock enable control"]
    #[inline(always)]
    pub fn spi_2_hclk_en(&self) -> SPI__HCLK_EN_R {
        SPI__HCLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI host \\[i\\]
slave AHB interface clock enable control"]
    #[inline(always)]
    pub fn spi_3_hclk_en(&self) -> SPI__HCLK_EN_R {
        SPI__HCLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 5 - I2S 2 slave APB interface clock enable control"]
    #[inline(always)]
    pub fn i2s_2_pclk_en(&self) -> I2S_2_PCLK_EN_R {
        I2S_2_PCLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "UART host \\[i\\]
slave APB interface clock enable control"]
    #[inline(always)]
    pub unsafe fn uart__pclk_en(&self, n: u8) -> UART__PCLK_EN_R {
        UART__PCLK_EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - UART host \\[i\\]
slave APB interface clock enable control"]
    #[inline(always)]
    pub fn uart_0_pclk_en(&self) -> UART__PCLK_EN_R {
        UART__PCLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART host \\[i\\]
slave APB interface clock enable control"]
    #[inline(always)]
    pub fn uart_1_pclk_en(&self) -> UART__PCLK_EN_R {
        UART__PCLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART host \\[i\\]
slave APB interface clock enable control"]
    #[inline(always)]
    pub fn uart_2_pclk_en(&self) -> UART__PCLK_EN_R {
        UART__PCLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART host \\[i\\]
slave APB interface clock enable control"]
    #[inline(always)]
    pub fn uart_3_pclk_en(&self) -> UART__PCLK_EN_R {
        UART__PCLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - Write enable for bit 10 (audif_pclk_en)"]
    #[inline(always)]
    pub fn we_audif_pclk_en(&mut self) -> WE_AUDIF_PCLK_EN_W {
        WE_AUDIF_PCLK_EN_W::new(self)
    }
    #[doc = "Write enable for bit 6-9 (spi_\\[0-3\\]_hclk_en)"]
    #[inline(always)]
    pub unsafe fn we_spi__hclk_en<const O: u8>(&mut self) -> WE_SPI__HCLK_EN_W<O> {
        WE_SPI__HCLK_EN_W::new(self)
    }
    #[doc = "Bit 22 - Write enable for bit 6-9 (spi_\\[0-3\\]_hclk_en)"]
    #[inline(always)]
    pub fn we_spi_0_hclk_en(&mut self) -> WE_SPI__HCLK_EN_W<22> {
        WE_SPI__HCLK_EN_W::new(self)
    }
    #[doc = "Bit 23 - Write enable for bit 6-9 (spi_\\[0-3\\]_hclk_en)"]
    #[inline(always)]
    pub fn we_spi_1_hclk_en(&mut self) -> WE_SPI__HCLK_EN_W<23> {
        WE_SPI__HCLK_EN_W::new(self)
    }
    #[doc = "Bit 24 - Write enable for bit 6-9 (spi_\\[0-3\\]_hclk_en)"]
    #[inline(always)]
    pub fn we_spi_2_hclk_en(&mut self) -> WE_SPI__HCLK_EN_W<24> {
        WE_SPI__HCLK_EN_W::new(self)
    }
    #[doc = "Bit 25 - Write enable for bit 6-9 (spi_\\[0-3\\]_hclk_en)"]
    #[inline(always)]
    pub fn we_spi_3_hclk_en(&mut self) -> WE_SPI__HCLK_EN_W<25> {
        WE_SPI__HCLK_EN_W::new(self)
    }
    #[doc = "Bit 21 - Write enable for bit 5 (i2s_2_pclk_en)"]
    #[inline(always)]
    pub fn we_i2s_2_pclk_en(&mut self) -> WE_I2S_2_PCLK_EN_W {
        WE_I2S_2_PCLK_EN_W::new(self)
    }
    #[doc = "Write enable for bit 0-3 (uart_\\[0-3\\]_pclk_en)"]
    #[inline(always)]
    pub unsafe fn we_uart__pclk_en<const O: u8>(&mut self) -> WE_UART__PCLK_EN_W<O> {
        WE_UART__PCLK_EN_W::new(self)
    }
    #[doc = "Bit 16 - Write enable for bit 0-3 (uart_\\[0-3\\]_pclk_en)"]
    #[inline(always)]
    pub fn we_uart_0_pclk_en(&mut self) -> WE_UART__PCLK_EN_W<16> {
        WE_UART__PCLK_EN_W::new(self)
    }
    #[doc = "Bit 17 - Write enable for bit 0-3 (uart_\\[0-3\\]_pclk_en)"]
    #[inline(always)]
    pub fn we_uart_1_pclk_en(&mut self) -> WE_UART__PCLK_EN_W<17> {
        WE_UART__PCLK_EN_W::new(self)
    }
    #[doc = "Bit 18 - Write enable for bit 0-3 (uart_\\[0-3\\]_pclk_en)"]
    #[inline(always)]
    pub fn we_uart_2_pclk_en(&mut self) -> WE_UART__PCLK_EN_W<18> {
        WE_UART__PCLK_EN_W::new(self)
    }
    #[doc = "Bit 19 - Write enable for bit 0-3 (uart_\\[0-3\\]_pclk_en)"]
    #[inline(always)]
    pub fn we_uart_3_pclk_en(&mut self) -> WE_UART__PCLK_EN_W<19> {
        WE_UART__PCLK_EN_W::new(self)
    }
    #[doc = "Bit 10 - Audio data interface slave APB port clock enable control"]
    #[inline(always)]
    pub fn audif_hclk_en(&mut self) -> AUDIF_HCLK_EN_W {
        AUDIF_HCLK_EN_W::new(self)
    }
    #[doc = "SPI host \\[i\\]
slave AHB interface clock enable control"]
    #[inline(always)]
    pub unsafe fn spi__hclk_en<const O: u8>(&mut self) -> SPI__HCLK_EN_W<O> {
        SPI__HCLK_EN_W::new(self)
    }
    #[doc = "Bit 6 - SPI host \\[i\\]
slave AHB interface clock enable control"]
    #[inline(always)]
    pub fn spi_0_hclk_en(&mut self) -> SPI__HCLK_EN_W<6> {
        SPI__HCLK_EN_W::new(self)
    }
    #[doc = "Bit 7 - SPI host \\[i\\]
slave AHB interface clock enable control"]
    #[inline(always)]
    pub fn spi_1_hclk_en(&mut self) -> SPI__HCLK_EN_W<7> {
        SPI__HCLK_EN_W::new(self)
    }
    #[doc = "Bit 8 - SPI host \\[i\\]
slave AHB interface clock enable control"]
    #[inline(always)]
    pub fn spi_2_hclk_en(&mut self) -> SPI__HCLK_EN_W<8> {
        SPI__HCLK_EN_W::new(self)
    }
    #[doc = "Bit 9 - SPI host \\[i\\]
slave AHB interface clock enable control"]
    #[inline(always)]
    pub fn spi_3_hclk_en(&mut self) -> SPI__HCLK_EN_W<9> {
        SPI__HCLK_EN_W::new(self)
    }
    #[doc = "Bit 5 - I2S 2 slave APB interface clock enable control"]
    #[inline(always)]
    pub fn i2s_2_pclk_en(&mut self) -> I2S_2_PCLK_EN_W {
        I2S_2_PCLK_EN_W::new(self)
    }
    #[doc = "UART host \\[i\\]
slave APB interface clock enable control"]
    #[inline(always)]
    pub unsafe fn uart__pclk_en<const O: u8>(&mut self) -> UART__PCLK_EN_W<O> {
        UART__PCLK_EN_W::new(self)
    }
    #[doc = "Bit 0 - UART host \\[i\\]
slave APB interface clock enable control"]
    #[inline(always)]
    pub fn uart_0_pclk_en(&mut self) -> UART__PCLK_EN_W<0> {
        UART__PCLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - UART host \\[i\\]
slave APB interface clock enable control"]
    #[inline(always)]
    pub fn uart_1_pclk_en(&mut self) -> UART__PCLK_EN_W<1> {
        UART__PCLK_EN_W::new(self)
    }
    #[doc = "Bit 2 - UART host \\[i\\]
slave APB interface clock enable control"]
    #[inline(always)]
    pub fn uart_2_pclk_en(&mut self) -> UART__PCLK_EN_W<2> {
        UART__PCLK_EN_W::new(self)
    }
    #[doc = "Bit 3 - UART host \\[i\\]
slave APB interface clock enable control"]
    #[inline(always)]
    pub fn uart_3_pclk_en(&mut self) -> UART__PCLK_EN_W<3> {
        UART__PCLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral subsystem modules bus IF clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_sys_bus_clk_en](index.html) module"]
pub struct PERI_SYS_BUS_CLK_EN_SPEC;
impl crate::RegisterSpec for PERI_SYS_BUS_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peri_sys_bus_clk_en::R](R) reader structure"]
impl crate::Readable for PERI_SYS_BUS_CLK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peri_sys_bus_clk_en::W](W) writer structure"]
impl crate::Writable for PERI_SYS_BUS_CLK_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERI_SYS_BUS_CLK_EN to value 0x07ef"]
impl crate::Resettable for PERI_SYS_BUS_CLK_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07ef
    }
}
