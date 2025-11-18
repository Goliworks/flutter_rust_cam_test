import 'package:flutter/material.dart';

class CustomButton extends StatelessWidget {
  final String? imagePath;
  final String? text;
  final VoidCallback? onPressed;

  const CustomButton({super.key, this.imagePath, this.text, this.onPressed});

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onPressed,
      child: Container(
        width: 150,
        height: 120,
        decoration: BoxDecoration(
          color: Colors.grey[300], // gris clair
          border: Border.all(
            color: Colors.grey[500]!, // gris foncé
            width: 1,
          ),
        ),
        child: Center(
          child: imagePath != null && imagePath!.isNotEmpty
              ? Image.asset(imagePath!, fit: BoxFit.cover)
              : Text(
                  text ?? 'No image',
                  style: TextStyle(color: Colors.black54, fontSize: 18),
                ),
        ),
      ),
    );
  }
}
