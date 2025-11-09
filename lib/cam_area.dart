// import 'dart:typed_data';

import 'package:flutter/material.dart';
// import 'package:flutter_rust_cam_test/src/rust/api/simple.dart';
import 'package:flutter_rust_cam_test/src/rust/api/camera.dart';
// import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_common.dart';
import 'package:flutter_webrtc/flutter_webrtc.dart';

class CamArea extends StatefulWidget {
  const CamArea({super.key});

  @override
  State<CamArea> createState() => _CamAreaState();
}

class _CamAreaState extends State<CamArea> {
  List<DropdownMenuEntry<String>> dropdownList = [];
  String? selectedItem;

  @override
  void initState() {
    super.initState();
    _initCams();
  }

  void _initCams() {
    initCams();

    checkForCameras().then((List<Cameras> cams) {
      for (Cameras cam in cams) {
        dropdownList.add(DropdownMenuEntry(label: cam.name, value: cam.id));
      }
      setState(() {
        dropdownList = cams.map((cam) {
          return DropdownMenuEntry(label: cam.name, value: cam.id);
        }).toList();
      });
    });
  }

  RTCVideoRenderer? _renderer;
  // MediaStream? _stream;

  // void _openCamera() async {
  //   // create and initialize renderer
  //   _renderer ??= RTCVideoRenderer();
  //   await _renderer!.initialize();
  //
  //   //
  //   try {
  //     _stream = await navigator.mediaDevices.getUserMedia({
  //       'audio': false,
  //       'video': true,
  //     });
  //   } catch (e) {
  //     //if you get an error, please check the permissions in the project settings.
  //     print(e.toString());
  //   }
  //
  //   // set the MediaStream to the video renderer
  //   _renderer!.srcObject = _stream;
  //   setState(() {});
  // }

  void _streamCam() {
    if (selectedItem == null) return;
    print("value: $selectedItem");
    streamCamera(id: int.parse(selectedItem!));
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

        // Uint8List? image;
        //
        // try {
        //   image = getImage(file: "/tmp/test.jpg");
        // } catch (e) {
        //   print(e);
        // }

        // limit to 1280x720
        width = width.clamp(0, maxWidth);
        height = height.clamp(0, maxHeight);

        return Center(
          child: Column(
            children: [
              Container(
                width: width,
                height: height,
                color: Colors.blueGrey,
                child: SizedBox(
                  // render the video renderer in the widget tree
                  child: _renderer != null
                      ? RTCVideoView(_renderer!)
                      : Container(),
                ),
              ),
              Padding(
                padding: const EdgeInsets.all(16.0),
                // child: FilledButton(
                //   onPressed: _openCamera,
                //   child: const Text("Open Camera"),
                // ),
                child: Wrap(
                  spacing: 10.0,
                  alignment: WrapAlignment.center,
                  crossAxisAlignment: WrapCrossAlignment.center,
                  children: [
                    DropdownMenu(
                      width: 300,
                      dropdownMenuEntries: dropdownList,
                      onSelected: (value) {
                        setState(() {
                          selectedItem = value!;
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
