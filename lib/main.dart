import 'package:flutter/material.dart';
import 'package:flutter_rust_cam_test/src/rust/api/simple.dart';
import 'package:flutter_rust_cam_test/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('Test Flutter / Rust cam')),
        body: LayoutBuilder(
          builder: (context, constraints) {
            const double maxWidth = 1200;
            const double maxHeight = 720;
            const double aspectRatio = maxWidth / maxHeight;

            double availableWidth = constraints.maxWidth;
            double availableHeight = constraints.maxHeight;

            double width = availableWidth;
            double height = width / aspectRatio;

            if (height > availableHeight) {
              height = availableHeight;
              width = height * aspectRatio;
            }

            // limit to 1280x720
            width = width.clamp(0, maxWidth);
            height = height.clamp(0, maxHeight);

            return Center(
              child: Container(
                width: width,
                height: height,
                color: Colors.blueGrey,
              ),
            );
          },
        ),
      ),
    );
  }
}
