library;

import 'package:flutter/foundation.dart';

/// 考评准则数据模型（v0.1 只读 API）。
///
/// 契约先行：字段以 `data/profile/qtrecurit/criteria.json` 为基准，
/// 与 Provider（`internal/model/criterion.go`）同源。
/// policy 与 assessment 承载长文（content），criterion 承载细则（description）。

/// Policy 是政策：知识提炼链产物的公开化展示。
@immutable
class Policy {
  final String id;
  final String title;
  final String content;
  final String status;

  const Policy({
    required this.id,
    required this.title,
    required this.content,
    required this.status,
  });

  factory Policy.fromJson(Map<String, dynamic> json) => Policy(
    id: json['id'] as String,
    title: json['title'] as String,
    content: json['content'] as String,
    status: (json['status'] as String?) ?? 'active',
  );
}

/// Criterion 是筛选标准：五个检验维度（期望匹配、动机视角、了解程度、学习意向、责任心）的落地细则。
@immutable
class Criterion {
  final String id;
  final String title;
  final String description;
  final String status;

  const Criterion({
    required this.id,
    required this.title,
    required this.description,
    required this.status,
  });

  factory Criterion.fromJson(Map<String, dynamic> json) => Criterion(
    id: json['id'] as String,
    title: json['title'] as String,
    description: json['description'] as String,
    status: (json['status'] as String?) ?? 'active',
  );
}

/// Assessment 是考核说明：考核分层、序列轨道、问卷与 AI 分析说明。
@immutable
class Assessment {
  final String id;
  final String title;
  final String content;
  final String status;

  const Assessment({
    required this.id,
    required this.title,
    required this.content,
    required this.status,
  });

  factory Assessment.fromJson(Map<String, dynamic> json) => Assessment(
    id: json['id'] as String,
    title: json['title'] as String,
    content: json['content'] as String,
    status: (json['status'] as String?) ?? 'active',
  );
}
