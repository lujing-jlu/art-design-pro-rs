#!/bin/bash

# API 测试脚本
BASE_URL="http://127.0.0.1:9123"

echo "=== 测试 1: 登录 ==="
LOGIN_RESPONSE=$(curl -s -X POST $BASE_URL/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"userName":"Super","password":"123456"}')

echo "$LOGIN_RESPONSE" | jq .

TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r .data.token)

if [ "$TOKEN" == "null" ] || [ -z "$TOKEN" ]; then
  echo "❌ 登录失败"
  exit 1
fi

echo "✅ 登录成功，获得 token"
echo ""

echo "=== 测试 2: 获取用户信息 ==="
curl -s -X GET $BASE_URL/api/user/info \
  -H "Authorization: Bearer $TOKEN" | jq .
echo ""

echo "=== 测试 3: 获取用户列表 ==="
curl -s -X GET "$BASE_URL/api/user/list?current=1&size=10" \
  -H "Authorization: Bearer $TOKEN" | jq .
echo ""

echo "=== 测试 4: 获取角色列表 ==="
curl -s -X GET "$BASE_URL/api/role/list?current=1&size=10" \
  -H "Authorization: Bearer $TOKEN" | jq .
echo ""

echo "=== 测试 5: 获取菜单列表 ==="
curl -s -X GET $BASE_URL/api/v3/system/menus/simple \
  -H "Authorization: Bearer $TOKEN" | jq .
echo ""

echo "✅ 所有测试完成！"
