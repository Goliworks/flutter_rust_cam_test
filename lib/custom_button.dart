import 'package:flutter/material.dart';

class CustomButton extends StatefulWidget {
  final String? imagePath;
  final String? text;
  final VoidCallback? onPressed;
  final bool? selected;
  const CustomButton({
    super.key,
    this.imagePath,
    this.text,
    this.onPressed,
    this.selected = false,
  });

  @override
  State<CustomButton> createState() => _CustomButtonState();
}

class _CustomButtonState extends State<CustomButton> {
  bool _hover = false;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) => setState(() => _hover = true),
      onExit: (_) => setState(() => _hover = false),
      child: InkWell(
        onTap: widget.onPressed,
        child: Container(
          width: 150,
          height: 120,
          decoration: BoxDecoration(
            color: Colors.grey[300],
            border: Border.all(
              color: widget.selected!
                  ? Colors.green
                  : _hover
                  ? Colors.orange
                  : Colors.grey,
              width: 1,
            ),
            boxShadow: [
              if (widget.selected!)
                BoxShadow(color: Colors.green, spreadRadius: 1)
              else if (_hover)
                BoxShadow(color: Colors.orange, spreadRadius: 1),
            ],
          ),
          child: ClipRRect(
            child: Align(
              alignment: Alignment.center,
              child: widget.imagePath != null && widget.imagePath!.isNotEmpty
                  ? Image.asset(
                      widget.imagePath!,
                      fit: BoxFit.cover,
                      width: 150,
                      height: 120,
                    )
                  : Text(
                      widget.text ?? 'No image',
                      style: TextStyle(color: Colors.black54, fontSize: 18),
                    ),
            ),
          ),
        ),
      ),
    );
  }
}
