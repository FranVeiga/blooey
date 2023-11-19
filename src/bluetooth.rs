use std::{future::Future, task::Context};

use bluer::{Address, Device, Result};

pub struct BluetoothManager {
    session: bluer::Session,
    adapter: bluer::Adapter,
}

#[derive(Clone)]
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

    async fn discovered_devices(&self) -> Result<Vec<Device>> {
        let address_list = self.adapter.device_addresses().await?;
        let mut devices = Vec::new();
        for address in address_list {
            let device = self.adapter.device(address)?;
            devices.push(device);
        }
        Ok(devices)
    }

    async fn device_to_device_list_item(&self, device: &Device) -> Result<DeviceListItem> {
        let address = device.address();
        let name = device.name().await?.unwrap_or(address.to_string());
        Ok(DeviceListItem {
            name,
            address,
            is_connected: device.is_connected().await?,
        })
    }

    pub async fn discovered_devices_list(&self) -> Result<Vec<DeviceListItem>> {
        let devices = self.discovered_devices().await?;
        let mut devices_list: Vec<DeviceListItem> = Vec::new();

        for device in devices.iter() {
            let list_item = self.device_to_device_list_item(&device).await?;
            devices_list.push(list_item);
        }
        Ok(devices_list)
    }

    pub async fn connect_device(&self, address: bluer::Address) -> Result<()> {
        let device = self.adapter.device(address)?;
        if device.is_paired().await? {
            device.connect().await?;
        }
        Ok(())
    }

    pub async fn disconnect_device(&self, address: bluer::Address) -> Result<()> {
        let device = self.adapter.device(address)?;
        device.disconnect().await
    }

    pub fn get_device(&self, item: &DeviceListItem) -> Result<Device> {
        self.adapter.device(item.address)
    }
}
