import 'package:flutter/material.dart';

/// Studio 占位首页。
///
/// v0.1 的四个页面（考评标准公开页、政策知识视图、候选人个人考评页、公平承诺页）
/// 见 [ROADMAP.md](../ROADMAP.md)，随迭代逐个落地。
class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('qtrecurit Studio')),
      body: const Center(child: Text('量潮招聘')),
    );
  }
}
