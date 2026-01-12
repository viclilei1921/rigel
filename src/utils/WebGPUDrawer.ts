export class WebGPUDrawer {
  private _canvas: HTMLCanvasElement
  private _device: GPUDevice | undefined
  private _context: GPUCanvasContext | undefined
  private _pipeline: GPURenderPipeline | undefined
  private _sampler: GPUSampler | undefined
  private _uniformBuffer: GPUBuffer | undefined
  // private _bindGroup: GPUBindGroup | undefined
  private _presentationFormat: GPUTextureFormat

  constructor(canvas: HTMLCanvasElement) {
    this._canvas = canvas
    this._presentationFormat = navigator.gpu.getPreferredCanvasFormat()
  }

  async init() {
    if (!navigator.gpu) {
      throw new Error('WebGPU is not supported')
    }

    const adapter = await navigator.gpu.requestAdapter()
    if (!adapter) {
      throw new Error('No GPU adapter found')
    }

    this._device = await adapter.requestDevice()
    this._context = this._canvas.getContext('webgpu') as GPUCanvasContext
    this._context.configure({
      device: this._device,
      format: this._presentationFormat,
      alphaMode: 'premultiplied'
    })

    // 创建通用采样器 (Linear Filter)
    this._sampler = this._device.createSampler({
      magFilter: 'linear',
      minFilter: 'linear',
      mipmapFilter: 'linear',
      addressModeU: 'clamp-to-edge',
      addressModeV: 'clamp-to-edge'
    })

    await this._initPipeline()
  }

  /** 编译 Shader 并创建管线 */
  async _initPipeline() {
    if (!this._device) {
      return
    }
    const shaderCode = `
        struct Uniforms { mvpMatrix : mat4x4<f32> }
        @group(0) @binding(0) var<uniform> uniforms : Uniforms;
        @group(0) @binding(1) var mySampler : sampler;
        @group(0) @binding(2) var myTexture : texture_2d<f32>;
        
        struct VertexOutput { @builtin(position) Position : vec4<f32>, @location(0) uv : vec2<f32> }
        
        @vertex
        fn vs_main(@builtin(vertex_index) vIdx : u32) -> VertexOutput {
            var pos = array<vec2<f32>, 6>(
                vec2(0., 0.), vec2(1., 0.), vec2(0., 1.),
                vec2(0., 1.), vec2(1., 0.), vec2(1., 1.)
            );
            var output : VertexOutput;
            output.Position = uniforms.mvpMatrix * vec4(pos[vIdx], 0.0, 1.0);
            output.uv = pos[vIdx]; // 纹理坐标
            return output;
        }

        @fragment
        fn fs_main(@location(0) uv : vec2<f32>) -> @location(0) vec4<f32> {
            return textureSample(myTexture, mySampler, uv);
        }
        `

    const shaderModule = this._device.createShaderModule({ code: shaderCode })

    // 创建 Uniform Buffer (用于存矩阵)
    // mat4x4<f32> = 16 float * 4 bytes = 64 bytes
    this._uniformBuffer = this._device.createBuffer({
      size: 64,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST
    })

    this._pipeline = this._device.createRenderPipeline({
      layout: 'auto',
      vertex: {
        module: shaderModule,
        entryPoint: 'vs_main'
      },
      fragment: {
        module: shaderModule,
        entryPoint: 'fs_main',
        targets: [{ format: this._presentationFormat }]
      },
      primitive: { topology: 'triangle-list' }
    })
  }

  async createTextureFromImage(source: HTMLCanvasElement | ImageBitmap | ImageData | HTMLImageElement | OffscreenCanvas) {
    if (!this._device) {
      return
    }

    const texture = this._device.createTexture({
      size: [source.width, source.height],
      format: 'rgba8unorm',
      usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST | GPUTextureUsage.RENDER_ATTACHMENT
    })

    this._device.queue.copyExternalImageToTexture(
      { source, flipY: false },
      { texture },
      [source.width, source.height]
    )

    return texture
  }

  /** 将 (x, y) 像素坐标转换为 WebGPU 的 (-1, 1) 坐标 */
  private _updateMatrix(x: number, y: number, width: number, height: number) {
    if (!this._device || !this._uniformBuffer) {
      return
    }

    const canvasW = this._canvas.width
    const canvasH = this._canvas.height

    // 标准正交投影公式 (Orthographic Projection)
    // 目标：将矩形 [x, x+w] 映射到 [-1, 1]

    // 简化版：直接计算 Model-View-Projection 矩阵
    // Scale (缩放)
    // Y轴翻转，因为 WebGPU Y轴向上为正(Clip Space)，但屏幕像素向下为正
    const sx = (2 * width) / canvasW
    const sy = (-2 * height) / canvasH

    // Translation (平移)
    // 需要将 0,0 (左上角) 移动到 -1, 1 (WebGPU 左上角)
    const tx = (2 * x) / canvasW - 1
    const ty = 1 - (2 * y) / canvasH

    // 构造 4x4 矩阵 (Column Major)
    const matrix = new Float32Array([
      sx,
      0,
      0,
      0,
      0,
      sy,
      0,
      0,
      0,
      0,
      1,
      0,
      tx,
      ty,
      0,
      1
    ])

    // 写入 GPU Buffer
    this._device.queue.writeBuffer(this._uniformBuffer, 0, matrix)
  }

  draw(texture: GPUTexture, x: number, y: number, width: number, height: number) {
    if (!this._pipeline || !this._device || !this._uniformBuffer || !this._sampler || !this._context) {
      return
    }

    // 1. 更新矩阵 uniform
    this._updateMatrix(x, y, width, height)

    // 2. 创建 BindGroup (将资源绑定到 Shader)
    // 在 WebGPU 中，这是每帧可能变化的部分
    const bindGroup = this._device.createBindGroup({
      layout: this._pipeline.getBindGroupLayout(0),
      entries: [
        { binding: 0, resource: { buffer: this._uniformBuffer } },
        { binding: 1, resource: this._sampler },
        { binding: 2, resource: texture.createView() }
      ]
    })

    // 3. 开始渲染通道
    const commandEncoder = this._device.createCommandEncoder()
    const textureView = this._context.getCurrentTexture().createView()

    const renderPassDescriptor: GPURenderPassDescriptor = {
      colorAttachments: [{
        view: textureView,
        clearValue: { r: 0, g: 0, b: 0, a: 0 }, // 清除背景色
        loadOp: 'clear', // 每一帧先清空。如果是画多个图，第二张图以后要改成 'load'
        storeOp: 'store'
      }]
    }

    const passEncoder = commandEncoder.beginRenderPass(renderPassDescriptor)
    passEncoder.setPipeline(this._pipeline)
    passEncoder.setBindGroup(0, bindGroup)
    // 绘制 6 个顶点 (2 个三角形)
    passEncoder.draw(6)
    passEncoder.end()

    // 4. 提交给 GPU 队列
    this._device.queue.submit([commandEncoder.finish()])

    // 如果你要在一帧画多张图，需要修改 draw 逻辑，把 BeginRenderPass 提到外面
    // 这里为了演示简单，每次 draw 都是一个完整的 pass
  }
}
