-- 用于添加一些固定的数据 如模块等
-- 模块表
-- 1.信息 2.权限 3.招聘 4.薪资 5.账户 6.绩效 7.考勤 8.消息
INSERT INTO modules (id, type)
VALUES (1, 'information'),
       (2, 'permission'),
       (3, 'recruitment'),
       (4, 'salary'),
       (5, 'account'),
       (6, 'performance'),
       (7, 'attendance'),
       (8, 'message');
-- 子模块表
INSERT INTO sub_modules (id, module_id, type)
VALUES (1, 1, 'all'),
       (2, 2, 'all'),
       (3, 3, 'all'),
       (4, 4, 'all'),
       (5, 5, 'all'),
       (6, 6, 'all'),
       (7, 7, 'all'),
       (8, 8, 'all');
-- 角色表
-- 1.管理员 2.员工 3.HR 4.求职者
INSERT INTO roles (id, type)
VALUES (1, 'admin'),
       (2, 'employee'),
       (3, 'hr'),
       (4, 'job_seeker');
-- 权限表
INSERT INTO permissions (role_id, module_id, sub_module_id)
VALUES (1, 1, 1),
       (1, 2, 2),
       (1, 3, 3),
       (1, 4, 4),
       (1, 5, 5),
       (1, 6, 6),
       (1, 7, 7),
       (1, 8, 8),
       (2, 1, 1),
       (2, 3, 3),
       (2, 4, 4),
       (2, 5, 5),
       (2, 6, 6),
       (2, 7, 7),
       (2, 8, 8),
       (3, 1, 1),
       (3, 2, 2),
       (3, 3, 3),
       (3, 4, 4),
       (3, 5, 5),
       (3, 6, 6),
       (3, 7, 7),
       (3, 8, 8),
       (4, 3, 1);
-- 职位表
-- 1.前端工程师 2.后端工程师 3.UI设计师 4.产品经理 5.HR 6.运营 7.销售 8.市场 9.财务 10.客服 11.运维工程师 12.测试工程师
INSERT INTO posts (id, type)
VALUES (1, 'font_end_engineer'),
       (2, 'back_end_engineer'),
       (3, 'ui_designer'),
       (4, 'product_manager'),
       (5, 'hr'),
       (6, 'operations'),
       (7, 'sales'),
       (8, 'marketing'),
       (9, 'finance'),
       (10, 'customer_service'),
       (11, 'operation_and_maintenance_engineer'),
       (12, 'test_engineer');
-- 公司表
-- 1.总公司 2.集团 3.管理公司 4.骨干支持 5.战略资源 6.门店
INSERT INTO companies (id, company_name)
VALUES (1, 'Gather ''n Simmer Inc.'),
       (2, 'Gather ''n Simmer Corporate Services'),
       (3, 'Gather ''n Simmer Management Solutions'),
       (4, 'Gather ''n Simmer Backbone Support'),
       (5, 'Gather ''n Simmer Strategic Resources'),
       (6, 'Gather ''n Simmer Store');
-- 门店表
INSERT INTO stores (id, store_name)
VALUES (1, 'Gather ''n Simmer Wangfujing'),
       (2, 'Gather ''n Simmer Sanlitun'),
       (3, 'Gather ''n Simmer Zhongguancun'),
       (4, 'Gather ''n Simmer Xidan'),
       (5, 'Gather ''n Simmer Chaoyang Park'),
       (6, 'Gather ''n Simmer Shuangjing'),
       (7, 'Gather ''n Simmer Guomao'),
       (8, 'Gather ''n Simmer Wangjing'),
       (9, 'Gather ''n Simmer Shichahai'),
       (10, 'Gather ''n Simmer Yizhuang');
-- 部门表
-- 1.研发部 2.HR部 3.运营部 4.销售部 5.市场部 6.财务部 7.客服部 8.运维部
INSERT INTO departments (id, department_name)
VALUES (1, 'R&D'),
       (2, 'HR'),
       (3, 'Operations'),
       (4, 'Sales'),
       (5, 'Marketing'),
       (6, 'Finance'),
       (7, 'Customer Service'),
       (8, 'Operation and Maintenance');