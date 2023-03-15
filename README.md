# Jfpng

采用微软新版必应编写.为了试验chat GPT可用性.

用来将jfif图片转换为png图片.

## 如何转换图标
运行`windres`编译资源文件
```
windres icon.rc -O coff -o icon.res
```
## 编译程序
```
cargo build -r
```

## 使用
如果手动运行需要导入注册表,修改相关路径即可.

如果你有编译程序,在jfif图片上点击右键,选择"Convert to png",即可运行程序,转换后会在相同目录下生成png文件.