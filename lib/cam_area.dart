import 'dart:async';
import 'dart:typed_data';
import 'dart:ui' as ui;
import 'package:flutter/material.dart';
// import 'package:flutter_rust_cam_test/src/rust/api/simple.dart';
import 'package:flutter_rust_cam_test/src/rust/api/camera.dart';
// import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_common.dart';

class CamArea extends StatefulWidget {
  const CamArea({super.key});

  @override
  State<CamArea> createState() => _CamAreaState();
}

class _CamAreaState extends State<CamArea> {
  List<DropdownMenuEntry<String>> _dropdownList = [];
  String? _selectedItem;

  Stream<Uint8List>? _camStream;

  @override
  void initState() {
    super.initState();
    _initCams();
  }

  void _initCams() {
    initCams();

    checkForCameras().then((List<Cameras> cams) {
      for (Cameras cam in cams) {
        _dropdownList.add(DropdownMenuEntry(label: cam.name, value: cam.id));
      }
      setState(() {
        _dropdownList = cams.map((cam) {
          return DropdownMenuEntry(label: cam.name, value: cam.id);
        }).toList();
      });
    });
  }

  void _streamCam() {
    if (_selectedItem == null) return;
    print("value: $_selectedItem");
    setState(() {
      _camStream = streamCamera(id: int.parse(_selectedItem!));
    });
  }

  Future<ui.Image> _createImage(Uint8List rgba) async {
    final completer = Completer<ui.Image>();
    ui.decodeImageFromPixels(
      rgba,
      640,
      480,
      ui.PixelFormat.rgba8888,
      completer.complete,
    );
    return completer.future;
  }

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
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
          child: Column(
            children: [
              Container(
                width: width,
                height: height,
                color: Colors.black,
                child: StreamBuilder(
                  stream: _camStream,
                  builder: (context, snapshot) {
                    if (!snapshot.hasData) {
                      return const Center(
                        child: Text(
                          "Waiting for webcam data.",
                          style: TextStyle(color: Colors.white),
                        ),
                      );
                    }

                    return FutureBuilder(
                      future: _createImage(snapshot.data!),
                      builder: (context, snapshot) {
                        return RawImage(
                          image: snapshot.data!,
                          fit: BoxFit.contain,
                        );
                      },
                    );
                  },
                ),
              ),
              Padding(
                padding: const EdgeInsets.all(16.0),
                child: Wrap(
                  spacing: 10.0,
                  alignment: WrapAlignment.center,
                  crossAxisAlignment: WrapCrossAlignment.center,
                  children: [
                    DropdownMenu(
                      width: 300,
                      dropdownMenuEntries: _dropdownList,
                      onSelected: (value) {
                        setState(() {
                          _selectedItem = value!;
                        });
                      },
                    ),
                    FilledButton(
                      onPressed: _streamCam,
                      child: const Text("Open Camera"),
                    ),
                  ],
                ),
              ), // add some padding)
            ],
          ),
        );
      },
    );
  }
}
