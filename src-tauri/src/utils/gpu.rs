use serde::{Deserialize, Serialize};
use wgpu::{Backends, Instance, InstanceDescriptor};

#[derive(Serialize, Deserialize, Debug)]
pub struct GpuInfo {
  pub name: String,
  pub backend: String,     // 如 Vulkan, Metal, DirectX 12
  pub device_type: String, // 如 "DiscreteGpu" 或 "IntegratedGpu" 等
}

pub async fn get_gpu_info() -> Result<Vec<GpuInfo>, String> {
  // 1. 创建实例，指定需要检测的后端（这里检测所有）
  let instance = Instance::new(&InstanceDescriptor {
    backends: Backends::all(), // 或指定 Backends::VULKAN | Backends::METAL 等[citation:3]
    ..Default::default()
  });

  // 2. 获取所有可用的适配器
  let adapters = instance.enumerate_adapters(Backends::all());
  let mut gpu_list = Vec::new();

  for adapter in adapters {
    // 3. 从适配器获取信息
    let info = adapter.get_info();

    // 4. 判断显卡类型（基于名称和设备类型的启发式方法）
    let device_type = if info.device_type == wgpu::DeviceType::DiscreteGpu {
      "DiscreteGpu".to_string()
    } else if info.device_type == wgpu::DeviceType::IntegratedGpu {
      "IntegratedGpu".to_string()
    } else if info.device_type == wgpu::DeviceType::VirtualGpu {
      "VirtualGpu".to_string()
    } else if info.device_type == wgpu::DeviceType::Cpu {
      "Cpu".to_string()
    } else {
      "Other".to_string()
    };

    // 5. 收集信息
    gpu_list.push(GpuInfo {
      name: info.name,
      backend: format!("{:?}", info.backend), // 将后端枚举转换为字符串
      device_type,
    });
  }

  if gpu_list.is_empty() {
    Err("未检测到可用的图形适配器。".into())
  } else {
    Ok(gpu_list)
  }
}
