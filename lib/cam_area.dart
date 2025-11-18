import 'dart:async';
import 'dart:typed_data';
import 'dart:ui' as ui;
import 'package:flutter/material.dart';
import 'package:flutter_rust_cam_test/effect_list.dart';
import 'package:flutter_rust_cam_test/effets_model.dart';
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

  ui.Image? _currentImage;
  StreamSubscription? _streamSubscription; // Pour gérer la souscription

  bool _hasMask = false;
  bool _debugMode = false;

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
    setMask(mask: _hasMask);
    final stream = streamCamera(id: int.parse(_selectedItem!));
    setState(() {
      _camStream = stream;
    });
    _startListening(stream);
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

  void _startListening(Stream<Uint8List> stream) {
    _streamSubscription?.cancel();

    _streamSubscription = stream.listen((Uint8List rgbaData) async {
      final newImage = await _createImage(rgbaData);

      // call setState only if the image has changed.
      if (newImage != _currentImage) {
        setState(() {
          // free old image.
          _currentImage?.dispose();
          _currentImage = newImage;
        });
      }
    });
  }

  void _stopCam() {
    _streamSubscription?.cancel();
    _currentImage?.dispose();

    setState(() {
      _camStream = null;
      _currentImage = null;
    });
  }

  void _changeMask(EffectsModel effects) {
    setState(() {
      _hasMask = effects.hasMask;
    });
    if (effects.background == null) {
      setMask(mask: _hasMask);
    } else {
      setBackground(background: effects.background!);
    }
  }

  void _changeDebug(bool debug) {
    setState(() {
      _debugMode = debug;
    });
    setDebug(debug: _debugMode);
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
                  child: _currentImage != null
                      ? RawImage(image: _currentImage, fit: BoxFit.contain)
                      : const Center(child: CircularProgressIndicator()),
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
                        onPressed: _stopCam,
                        style: ButtonStyle(
                          backgroundColor: WidgetStatePropertyAll(Colors.red),
                        ),
                        child: const Text("Stop Camera"),
                      ),
                  ],
                ),
              ), // add some padding)
              EffectList(onChanged: (val) => _changeMask(val)),
              Wrap(
                spacing: 10.0,
                alignment: WrapAlignment.center,
                crossAxisAlignment: WrapCrossAlignment.center,
                children: [
                  Checkbox(
                    value: _debugMode,
                    onChanged: (value) => _changeDebug(value!),
                  ),
                  const Text("Debug mode"),
                ],
              ),
            ],
          ),
        );
      },
    );
  }

  @override
  void dispose() {
    _streamSubscription?.cancel();
    _currentImage?.dispose();
    super.dispose();
  }
}
