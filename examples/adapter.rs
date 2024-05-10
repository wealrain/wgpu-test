fn main() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    instance.enumerate_adapters(wgpu::Backends::all()).into_iter().for_each(|adapter| {
        println!("{:?}", adapter.get_info());
        // 支持的特定功能
        adapter.features().iter().for_each(|feature| {
            println!("{:?}", feature);
        });

        
    });
}