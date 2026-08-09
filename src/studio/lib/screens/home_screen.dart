import 'package:flutter/material.dart';

import 'recommendation_screen.dart';

/// Studio 首页：推荐信查询入口。
///
/// v0.1 提供示例推荐信查看；凭证查询与分享链接随迭代落地。
class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    return Scaffold(
      appBar: AppBar(title: const Text('量潮招聘')),
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(
                Icons.workspace_premium,
                size: 64,
                color: colorScheme.primary,
              ),
              const SizedBox(height: 16),
              Text('量潮招聘', style: Theme.of(context).textTheme.headlineSmall),
              const SizedBox(height: 8),
              Text(
                '结构化推荐信：客观行为记录与我们的评价',
                style: TextStyle(color: colorScheme.outline),
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 24),
              FilledButton.icon(
                onPressed: () {
                  Navigator.of(context).push(
                    MaterialPageRoute(
                      builder: (_) => const RecommendationScreen(),
                    ),
                  );
                },
                icon: const Icon(Icons.article),
                label: const Text('查看示例推荐信'),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
