#!/bin/bash

# 测试用户中心相关接口
# 需要先启动后端服务: cargo run -p api

BASE_URL="http://127.0.0.1:9123/api"

echo "=== 测试用户中心接口 ==="
echo ""

# 1. 登录获取 token
echo "1. 登录..."
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/auth/login" \
  -H "Content-Type: application/json" \
  -d '{"userName":"Super","password":"123456"}')

TOKEN=$(echo $LOGIN_RESPONSE | jq -r '.data.token')
echo "Token: ${TOKEN:0:50}..."
echo ""

# 2. 获取用户信息
echo "2. 获取用户信息..."
curl -s -X GET "$BASE_URL/user/info" \
  -H "Authorization: Bearer $TOKEN" | jq '.'
echo ""

# 3. 更新个人信息
echo "3. 更新个人信息..."
curl -s -X PUT "$BASE_URL/user/profile" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "realName": "张三",
    "nickName": "超级管理员",
    "email": "super@example.com",
    "phone": "13800138000",
    "gender": "男",
    "address": "北京市朝阳区",
    "bio": "这是一个超级管理员账号"
  }' | jq '.'
echo ""

# 4. 再次获取用户信息验证更新
echo "4. 验证更新后的用户信息..."
curl -s -X GET "$BASE_URL/user/info" \
  -H "Authorization: Bearer $TOKEN" | jq '.'
echo ""

# 5. 修改密码（使用错误的旧密码）
echo "5. 测试修改密码（错误的旧密码）..."
curl -s -X PUT "$BASE_URL/user/password" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "oldPassword": "wrongpassword",
    "newPassword": "NewPass123"
  }' | jq '.'
echo ""

# 6. 修改密码（正确的旧密码）
echo "6. 测试修改密码（正确的旧密码）..."
curl -s -X PUT "$BASE_URL/user/password" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "oldPassword": "123456",
    "newPassword": "NewPass123"
  }' | jq '.'
echo ""

# 7. 用新密码登录
echo "7. 用新密码登录..."
curl -s -X POST "$BASE_URL/auth/login" \
  -H "Content-Type: application/json" \
  -d '{"userName":"Super","password":"NewPass123"}' | jq '.'
echo ""

# 8. 改回原密码
echo "8. 改回原密码..."
NEW_TOKEN=$(curl -s -X POST "$BASE_URL/auth/login" \
  -H "Content-Type: application/json" \
  -d '{"userName":"Super","password":"NewPass123"}' | jq -r '.data.token')

curl -s -X PUT "$BASE_URL/user/password" \
  -H "Authorization: Bearer $NEW_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "oldPassword": "NewPass123",
    "newPassword": "123456"
  }' | jq '.'
echo ""

echo "=== 测试完成 ==="
