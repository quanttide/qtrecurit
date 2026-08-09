import 'package:flutter/material.dart';

import 'screens/home_screen.dart';

void main() {
  runApp(const QtrecuritApp());
}

class QtrecuritApp extends StatelessWidget {
  const QtrecuritApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: '量潮招聘',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: const Color(0xFF4F46E5),
          surface: Colors.white,
          brightness: Brightness.light,
        ),
        scaffoldBackgroundColor: const Color(0xFFF1F5F9),
        useMaterial3: true,
      ),
      home: const HomeScreen(),
    );
  }
}
