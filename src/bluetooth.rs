use bluer::{Address, Result};

pub struct BluetoothManager {
    session: bluer::Session,
    adapter: bluer::Adapter,
}

pub struct DeviceListItem {
    pub name: String,
    pub address: Address,
    pub is_connected: bool,
}

impl BluetoothManager {
    pub async fn create() -> Result<BluetoothManager> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        Ok(BluetoothManager { session, adapter })
    }

    pub async fn discovered_devices_list(&self) -> Result<Vec<DeviceListItem>> {
        let address_list = self.adapter.device_addresses().await?;
        let mut device_item_list = Vec::new();
        for address in address_list {
            let device = self.adapter.device(address)?;
            if let Some(name) = device.name().await? {
                device_item_list.push(DeviceListItem {
                    name,
                    address,
                    is_connected: device.is_connected().await?,
                });
            }
        }
        Ok(device_item_list)
    }

    pub async fn connect_device(&self, device: bluer::Device) -> Result<()> {
        if device.is_paired().await? {
            device.connect().await?;
        }
        Ok(())
    }
}
