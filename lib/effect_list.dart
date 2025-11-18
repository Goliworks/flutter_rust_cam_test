import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_rust_cam_test/custom_button.dart';
import 'package:flutter_rust_cam_test/effets_model.dart';

class EffectList extends StatelessWidget {
  const EffectList({super.key, required this.onChanged});

  final void Function(EffectsModel) onChanged;

  void _blur() {
    final effects = EffectsModel();
    effects.hasMask = true;
    onChanged(effects);
  }

  Future<void> _background(String imageName) async {
    final data = await rootBundle.load("assets/images/$imageName.jpg");
    final buffer = data.buffer.asUint8List();

    final effects = EffectsModel();
    effects.background = buffer;
    onChanged(effects);
  }

  void _noEffect() {
    final effects = EffectsModel();
    onChanged(effects);
  }

  @override
  Widget build(BuildContext context) {
    return Wrap(
      spacing: 10,
      children: [
        CustomButton(onPressed: _noEffect, text: "No effect"),
        CustomButton(onPressed: _blur, text: "Blur"),
        CustomButton(
          onPressed: () => _background("mountains"),
          imagePath: "assets/images/mountains.jpg",
        ),
        CustomButton(
          onPressed: () => _background("office"),
          imagePath: "assets/images/office.jpg",
        ),
        CustomButton(
          onPressed: () => _background("city"),
          imagePath: "assets/images/city.jpg",
        ),
      ],
    );
  }
}
