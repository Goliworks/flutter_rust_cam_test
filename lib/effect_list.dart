import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_rust_cam_test/custom_button.dart';

class EffectList extends StatelessWidget {
  const EffectList({super.key, required this.onChanged});

  final void Function(bool) onChanged;
  void _noEffets() {
    print("no effect");
  }

  @override
  Widget build(BuildContext context) {
    return Wrap(
      spacing: 10,
      children: [
        CustomButton(onPressed: () => onChanged(false), text: "No effect"),
        CustomButton(onPressed: () => onChanged(true), text: "Blur"),
        CustomButton(
          onPressed: _noEffets,
          imagePath: "assets/images/mountains.jpg",
        ),
      ],
    );
  }
}
