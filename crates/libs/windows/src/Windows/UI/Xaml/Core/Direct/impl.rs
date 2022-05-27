pub trait IXamlDirectObject_Impl: Sized {}
impl ::windows_core::RuntimeName for IXamlDirectObject {
    const NAME: &'static str = "Windows.UI.Xaml.Core.Direct.IXamlDirectObject";
}
impl IXamlDirectObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXamlDirectObject_Impl, const OFFSET: isize>() -> IXamlDirectObject_Vtbl {
        Self { base__: ::windows_core::IInspectableVtbl::new::<Identity, IXamlDirectObject, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlDirectObject as ::windows_core::Interface>::IID
    }
}
