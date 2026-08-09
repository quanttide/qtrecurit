import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import '../models/recommendation.dart';

/// 结构化推荐信页面。
///
/// 内容 = 客观行为记录（事实可验证）+ 我们的评价（署名）。
/// 不展示考核标准与评估结论。
class RecommendationScreen extends StatefulWidget {
  final String recommendationId;

  const RecommendationScreen({super.key, this.recommendationId = 'rec_001'});

  @override
  State<RecommendationScreen> createState() => _RecommendationScreenState();
}

class _RecommendationScreenState extends State<RecommendationScreen> {
  Future<Recommendation>? _future;

  @override
  void initState() {
    super.initState();
    _future = _loadRecommendation();
  }

  Future<Recommendation> _loadRecommendation() async {
    final raw = await rootBundle.loadString('assets/recommendations.json');
    final list = jsonDecode(raw) as List<dynamic>;
    final rec = list
        .cast<Map<String, dynamic>>()
        .map(Recommendation.fromJson)
        .firstWhere((r) => r.id == widget.recommendationId);
    return rec;
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('结构化推荐信')),
      body: FutureBuilder<Recommendation>(
        future: _future,
        builder: (context, snapshot) {
          if (snapshot.connectionState != ConnectionState.done) {
            return const Center(child: CircularProgressIndicator());
          }
          if (snapshot.hasError) {
            return Center(child: Text('加载失败：${snapshot.error}'));
          }
          return _buildContent(context, snapshot.data!);
        },
      ),
    );
  }

  Widget _buildContent(BuildContext context, Recommendation rec) {
    final textTheme = Theme.of(context).textTheme;
    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        _buildHeaderCard(context, rec),
        const SizedBox(height: 24),
        Text('客观行为记录', style: textTheme.titleLarge),
        const SizedBox(height: 8),
        for (final b in rec.behaviors) _buildBehaviorCard(context, b),
        const SizedBox(height: 24),
        Text('我们的评价', style: textTheme.titleLarge),
        const SizedBox(height: 8),
        for (final e in rec.evaluations) _buildEvaluationCard(context, e),
        const SizedBox(height: 24),
        _buildVerificationCard(context, rec),
      ],
    );
  }

  Widget _buildHeaderCard(BuildContext context, Recommendation rec) {
    final colorScheme = Theme.of(context).colorScheme;
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              '推荐信',
              style: Theme.of(
                context,
              ).textTheme.titleLarge?.copyWith(color: colorScheme.primary),
            ),
            const SizedBox(height: 12),
            _headerRow('推荐方', rec.recommender),
            _headerRow('被推荐人', '${rec.candidate}（${rec.identity}）'),
            _headerRow('编号', rec.id),
            _headerRow('评估时点', rec.issuedAt),
          ],
        ),
      ),
    );
  }

  Widget _headerRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 2),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            width: 80,
            child: Text(
              label,
              style: TextStyle(color: Theme.of(context).colorScheme.outline),
            ),
          ),
          Expanded(child: Text(value)),
        ],
      ),
    );
  }

  Widget _buildBehaviorCard(BuildContext context, BehaviorRecord b) {
    final colorScheme = Theme.of(context).colorScheme;
    final isPractice = b.type == 'practice';
    return Card(
      child: ListTile(
        leading: Icon(
          isPractice ? Icons.workspace_premium : Icons.history,
          color: colorScheme.primary,
        ),
        title: Text(b.title),
        subtitle: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              b.time,
              style: TextStyle(color: colorScheme.outline, fontSize: 12),
            ),
            const SizedBox(height: 4),
            Text(b.result),
          ],
        ),
        isThreeLine: true,
      ),
    );
  }

  Widget _buildEvaluationCard(BuildContext context, Evaluation e) {
    final colorScheme = Theme.of(context).colorScheme;
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(e.content),
            const SizedBox(height: 12),
            Row(
              children: [
                CircleAvatar(
                  radius: 14,
                  backgroundColor: colorScheme.primaryContainer,
                  child: Text(
                    e.author.characters.first,
                    style: TextStyle(
                      fontSize: 12,
                      color: colorScheme.onPrimaryContainer,
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                Text(
                  '${e.author} · ${e.role}',
                  style: TextStyle(color: colorScheme.outline, fontSize: 12),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildVerificationCard(BuildContext context, Recommendation rec) {
    final colorScheme = Theme.of(context).colorScheme;
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(Icons.verified, size: 20, color: colorScheme.primary),
                const SizedBox(width: 8),
                Text('可验证', style: Theme.of(context).textTheme.titleMedium),
              ],
            ),
            const SizedBox(height: 8),
            Text(
              '企业可通过推荐信编号 ${rec.id} 向量潮招聘查证事实与评价。',
              style: TextStyle(color: colorScheme.outline),
            ),
            const SizedBox(height: 8),
            Text(
              '推荐信反映评估时点状态，非未来担保。',
              style: TextStyle(color: colorScheme.outline, fontSize: 12),
            ),
            const SizedBox(height: 12),
            FilledButton.tonalIcon(
              onPressed: () {
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(content: Text('PDF 导出将在 v0.2 提供')),
                );
              },
              icon: const Icon(Icons.ios_share),
              label: const Text('导出 PDF'),
            ),
          ],
        ),
      ),
    );
  }
}
