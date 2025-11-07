import 'package:flutter/material.dart';
import 'package:flutter_rust_cam_test/cam_area.dart';
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
        body: const CamArea(),
      ),
    );
  }
}
