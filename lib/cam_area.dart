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
  bool _isStreaming = false;

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
              if (_camStream != null)
                Container(
                  width: width,
                  height: height,
                  color: Colors.black,
                  child: StreamBuilder(
                    stream: _camStream,
                    builder: (context, snapshot) {
                      if (!snapshot.hasData) {
                        return const Center(child: CircularProgressIndicator());
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
                )
              else
                Container(
                  width: width,
                  height: height,
                  color: Colors.black,
                  child: const Center(
                    child: Text(
                      "No camera selected.",
                      style: TextStyle(color: Colors.white),
                    ),
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
                      enableSearch: false,
                      enableFilter: false,
                      dropdownMenuEntries: _dropdownList,
                      onSelected: (value) {
                        setState(() {
                          _selectedItem = value!;
                        });
                      },
                    ),
                    if (_camStream == null)
                      FilledButton(
                        onPressed: _streamCam,
                        style: ButtonStyle(
                          backgroundColor: WidgetStatePropertyAll(Colors.green),
                        ),
                        child: const Text("Start Camera"),
                      )
                    else
                      FilledButton(
                        onPressed: () {
                          setState(() {
                            _camStream = null;
                          });
                        },
                        style: ButtonStyle(
                          backgroundColor: WidgetStatePropertyAll(Colors.red),
                        ),
                        child: const Text("Stop Camera"),
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
