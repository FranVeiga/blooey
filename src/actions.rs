use crate::bluetooth::DeviceListItem;

pub enum Action {
    ChangeColor,
    Quit,
    Noop,
    Up,
    Down,
    Connect(DeviceListItem),
    Disconnect(DeviceListItem),
    SelectDeviceForConnection,
    SelectDeviceForDisconnection,
    RemoveAlert,
}
