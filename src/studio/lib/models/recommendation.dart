library;

import 'package:flutter/foundation.dart';

/// 推荐信数据模型（v0.1 本地 assets 演示，契约先行）。
///
/// 推荐信 = 客观行为记录（BehaviorRecord）+ 署名评价（Evaluation）。
/// 不包含考核标准与评估结论。

/// 客观行为记录：候选人在组织里实际做过的事情，事实可验证。
@immutable
class BehaviorRecord {
  final String id;
  final String type; // behavior | practice
  final String title;
  final String time;
  final String result;

  const BehaviorRecord({
    required this.id,
    required this.type,
    required this.title,
    required this.time,
    required this.result,
  });

  factory BehaviorRecord.fromJson(Map<String, dynamic> json) => BehaviorRecord(
    id: json['id'] as String,
    type: (json['type'] as String?) ?? 'behavior',
    title: json['title'] as String,
    time: json['time'] as String,
    result: json['result'] as String,
  );
}

/// 署名评价：评价者与身份为评价负责。
@immutable
class Evaluation {
  final String author;
  final String role;
  final String content;

  const Evaluation({
    required this.author,
    required this.role,
    required this.content,
  });

  factory Evaluation.fromJson(Map<String, dynamic> json) => Evaluation(
    author: json['author'] as String,
    role: json['role'] as String,
    content: json['content'] as String,
  );
}

/// 结构化推荐信：客观行为记录 + 我们的评价。
@immutable
class Recommendation {
  final String id;
  final String recommender;
  final String candidate;
  final String identity;
  final String issuedAt;
  final List<BehaviorRecord> behaviors;
  final List<Evaluation> evaluations;

  const Recommendation({
    required this.id,
    required this.recommender,
    required this.candidate,
    required this.identity,
    required this.issuedAt,
    required this.behaviors,
    required this.evaluations,
  });

  factory Recommendation.fromJson(Map<String, dynamic> json) => Recommendation(
    id: json['id'] as String,
    recommender: json['recommender'] as String,
    candidate: json['candidate'] as String,
    identity: json['identity'] as String,
    issuedAt: json['issuedAt'] as String,
    behaviors: (json['behaviors'] as List<dynamic>)
        .map((e) => BehaviorRecord.fromJson(e as Map<String, dynamic>))
        .toList(),
    evaluations: (json['evaluations'] as List<dynamic>)
        .map((e) => Evaluation.fromJson(e as Map<String, dynamic>))
        .toList(),
  );
}
