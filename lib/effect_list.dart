import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_rust_cam_test/custom_button.dart';
import 'package:flutter_rust_cam_test/effets_model.dart';

class _EffectButton {
  final String? text;
  final String? imagePath;
  final void Function(int e)? onPressed;

  _EffectButton({this.text, this.imagePath, this.onPressed});
}

class EffectList extends StatelessWidget {
  const EffectList({
    super.key,
    required this.onChanged,
    required this.selectedEffect,
  });

  final void Function(EffectsModel, int) onChanged;
  final int selectedEffect;

  void _blur(int e) {
    final effects = EffectsModel();
    effects.hasMask = true;
    onChanged(effects, e);
  }

  Future<void> _background(String imageName, int e) async {
    final data = await rootBundle.load("assets/images/$imageName.jpg");
    final buffer = data.buffer.asUint8List();

    final effects = EffectsModel();
    effects.background = buffer;
    onChanged(effects, e);
  }

  void _noEffect(int e) {
    final effects = EffectsModel();
    onChanged(effects, e);
  }

  @override
  Widget build(BuildContext context) {
    final buttons = [
      _EffectButton(text: "No effect", onPressed: _noEffect),
      _EffectButton(text: "Blur", onPressed: _blur),
      _EffectButton(
        imagePath: "assets/images/mountains.jpg",
        onPressed: (e) => _background("mountains", e),
      ),
      _EffectButton(
        imagePath: "assets/images/office.jpg",
        onPressed: (e) => _background("office", e),
      ),
      _EffectButton(
        imagePath: "assets/images/city.jpg",
        onPressed: (e) => _background("city", e),
      ),
    ];

    return Wrap(
      spacing: 10,
      children: buttons.asMap().entries.map(
        (e) {
          final index = e.key;
          final item = e.value;
          return CustomButton(
            text: item.text,
            imagePath: item.imagePath,
            selected: index == selectedEffect,
            onPressed: () {
              item.onPressed!(index);
            },
          );
        },
      ).toList(), // List.generate(buttons.length, (index) => buttons[index].toElement)   );
    );
  }
}
