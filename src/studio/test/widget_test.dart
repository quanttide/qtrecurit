// 冒烟测试：品牌入口与占位首页渲染。

import 'package:flutter_test/flutter_test.dart';

import 'package:qtrecurit_studio/main.dart';

void main() {
  testWidgets('App 冒烟测试：品牌主题与占位首页', (WidgetTester tester) async {
    await tester.pumpWidget(const QtrecuritApp());

    expect(find.text('qtrecurit Studio'), findsOneWidget);
    expect(find.text('量潮招聘'), findsOneWidget);
  });
}
