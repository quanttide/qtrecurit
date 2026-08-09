// 冒烟测试：品牌入口与推荐信页面。

import 'package:flutter_test/flutter_test.dart';

import 'package:qtrecurit_studio/main.dart';

void main() {
  testWidgets('App 冒烟测试：品牌主题与首页', (WidgetTester tester) async {
    await tester.pumpWidget(const QtrecuritApp());

    expect(find.text('量潮招聘'), findsNWidgets(2)); // AppBar + 正文标题
    expect(find.text('查看示例推荐信'), findsOneWidget);
  });

  testWidgets('推荐信页面：客观行为记录与署名评价', (WidgetTester tester) async {
    await tester.pumpWidget(const QtrecuritApp());
    await tester.tap(find.text('查看示例推荐信'));
    await tester.pumpAndSettle();

    expect(find.text('结构化推荐信'), findsOneWidget);
    expect(find.text('客观行为记录'), findsOneWidget);
    expect(find.text('我们的评价'), findsOneWidget);
    expect(find.text('维护招聘邮箱与候选人标签'), findsOneWidget);
    expect(find.text('李指导 · 直接指导者'), findsOneWidget);

    // 验证区位于列表底部，滚动后可见
    await tester.scrollUntilVisible(find.text('可验证'), 300);
    expect(find.text('可验证'), findsOneWidget);
  });
}
