-- Add migration script here
-- Add migration script here
DROP TABLE IF EXISTS apf_hi_identitylink;
DROP TABLE IF EXISTS apf_ru_identitylink;
DROP TABLE IF EXISTS apf_hi_varinst;
DROP TABLE IF EXISTS apf_ru_variable;
DROP TABLE IF EXISTS apf_hi_taskinst;
DROP TABLE IF EXISTS apf_ru_task;
DROP TABLE IF EXISTS apf_hi_procinst;
DROP TABLE IF EXISTS apf_hi_actinst;
DROP TABLE IF EXISTS apf_ru_execution;
DROP TABLE IF EXISTS apf_re_procdef;
DROP TABLE IF EXISTS apf_ge_bytearray;
DROP TABLE IF EXISTS apf_re_deployment;
DROP TABLE IF EXISTS test;

CREATE TABLE apf_re_deployment (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    key VARCHAR(255) NOT NULL,
    company_id VARCHAR(255) NOT NULL,
    company_name VARCHAR(100) NOT NULL,
    deployer_id VARCHAR(255) NOT NULL,
    deployer_name VARCHAR(50) NOT NULL,
    deploy_time BIGINT NOT NULL
);

CREATE TABLE apf_re_procdef (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    rev INT DEFAULT NULL,
    name VARCHAR(255) NOT NULL,
    key VARCHAR(255)  NOT NULL,
    version INT NOT NULL DEFAULT 1,
    deployment_id VARCHAR(255) NOT NULL REFERENCES apf_re_deployment(id),
    resource_name VARCHAR(4000) NULL,
    description VARCHAR(4000) NULL,
    suspension_state INT DEFAULT 0,
    company_id VARCHAR(255) NOT NULL,
    company_name VARCHAR(100) NOT NULL,
    deployer_id VARCHAR(255) NOT NULL,
    deployer_name VARCHAR(50) NOT NULL,
    is_deleted INT DEFAULT 0,

    unique (key, version)
);

CREATE TABLE apf_ge_bytearray (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NULL,
    deployment_id VARCHAR(255) NOT NULL REFERENCES apf_re_deployment(id),
    bytes BYTEA
);

CREATE TABLE apf_ru_execution (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    rev INT DEFAULT NULL,
    proc_inst_id VARCHAR(255) NULL REFERENCES apf_ru_execution(id),
    business_key VARCHAR(255) NULL,
    parent_id VARCHAR(255) NULL REFERENCES apf_ru_execution(id),
    proc_def_id VARCHAR(255) NULL REFERENCES apf_re_procdef(id),
    root_proc_inst_id VARCHAR(255) NULL,
    element_id VARCHAR(255) NULL,
    is_active INT DEFAULT 1,
    start_time BIGINT NOT NULL,
    start_user VARCHAR(255) NULL
);

CREATE TABLE apf_hi_actinst (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    rev INT DEFAULT NULL,
    proc_def_id VARCHAR(255) NOT NULL,
    proc_inst_id VARCHAR(255) NOT NULL,
    execution_id VARCHAR(255)  NOT NULL,
    task_id VARCHAR(255) NULL,
    element_id VARCHAR(255) NOT NULL,
    element_name VARCHAR(255) NULL,
    element_type VARCHAR(255) NULL,
    start_user_id VARCHAR(255) NULL,
    end_user_id VARCHAR(255) NULL,
    start_time BIGINT NOT NULL,
    end_time BIGINT NULL,
    duration BIGINT NULL
);
CREATE INDEX apf_idx_hi_act_inst_start ON apf_hi_actinst (start_time);
CREATE INDEX apf_idx_hi_act_inst_end ON apf_hi_actinst (end_time);
CREATE INDEX apf_idx_hi_act_inst_procinst ON apf_hi_actinst (proc_inst_id, element_id);
CREATE INDEX apf_idx_hi_act_inst_exec ON apf_hi_actinst (execution_id, element_id);

CREATE TABLE apf_hi_procinst (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    rev INT DEFAULT NULL,
    proc_inst_id VARCHAR(255) NOT NULL,
    business_key VARCHAR(255) NULL,
    proc_def_id VARCHAR(255) NOT NULL,
    start_time BIGINT NOT NULL,
    end_time BIGINT NULL,
    duration BIGINT NULL default 0,
    start_user VARCHAR(255) NULL,
    start_element_id VARCHAR(255) NULL,
    end_element_id VARCHAR(255) NULL,

    unique (proc_inst_id)
);
CREATE INDEX apf_idx_hi_pro_inst_end ON apf_hi_procinst (end_time);
CREATE INDEX apf_idx_hi_pro_i_buskey ON apf_hi_procinst (business_key);

CREATE TABLE apf_ru_task (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    rev INT DEFAULT NULL,
    execution_id VARCHAR(255) NOT NULL REFERENCES apf_ru_execution(id),
    proc_inst_id VARCHAR(255) NOT NULL REFERENCES apf_ru_execution(id),
    proc_def_id VARCHAR(255) NOT NULL REFERENCES apf_re_procdef(id),
    element_id VARCHAR(255) NOT NULL,
    element_name VARCHAR(255) NULL,
    element_type VARCHAR(255) NULL,
    business_key VARCHAR(255) NULL,
    description VARCHAR(4000) NULL,
    start_user_id VARCHAR(255) NULL,
    create_time BIGINT NOT NULL,
    suspension_state INT NOT NULL DEFAULT 0,
    form_key VARCHAR(255) NULL
);
CREATE INDEX apf_idx_task_create ON apf_ru_task (create_time);
CREATE INDEX apf_idx_task_exe ON apf_ru_task (execution_id);
CREATE INDEX apf_idx_task_procinst ON apf_ru_task (proc_inst_id);
CREATE INDEX apf_idx_task_procdef ON apf_ru_task (proc_def_id);

CREATE TABLE apf_hi_taskinst (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    rev INT DEFAULT NULL,
    execution_id VARCHAR(255) NOT NULL,
    proc_inst_id VARCHAR(255) NOT NULL,
    proc_def_id VARCHAR(255) NOT NULL,
    element_id VARCHAR(255) NOT NULL,
    element_name VARCHAR(255) NULL,
    element_type VARCHAR(255) NULL,
    business_key VARCHAR(255) NULL,
    description VARCHAR(4000) NULL,
    start_user_id VARCHAR(255) NULL,
    end_user_id VARCHAR(255) NULL,
    start_time BIGINT NOT NULL,
    end_time BIGINT NULL,
    duration BIGINT NULL,
    suspension_state INT NULL,
    form_key VARCHAR(255) NULL
);
CREATE INDEX apf_idx_hi_task_inst_procinst ON apf_hi_taskinst (proc_inst_id);

CREATE TABLE apf_ru_variable (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    rev INT DEFAULT NULL,
    var_type VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    execution_id VARCHAR(255) NULL,
    proc_inst_id VARCHAR(255) NOT NULL REFERENCES apf_ru_execution(id),
    task_id VARCHAR(255) NULL , -- 任务结束后，任务会被删除，但是变量需要留着给后续流程使用
    value VARCHAR(4000) NULL
);
CREATE INDEX apf_idx_variable_task_id ON apf_ru_variable (task_id);
CREATE INDEX apf_fk_var_exe ON apf_ru_variable (execution_id);
CREATE INDEX apf_fk_var_procinst ON apf_ru_variable (proc_inst_id);

CREATE TABLE apf_hi_varinst (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    rev INT DEFAULT NULL,
    var_type VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    execution_id VARCHAR(255) NULL ,
    proc_inst_id VARCHAR(255) NOT NULL,
    task_id VARCHAR(255) NULL,
    value VARCHAR(4000) NULL,
    create_time BIGINT NULL,
    last_updated_time BIGINT NULL
);
CREATE INDEX apf_idx_hi_procvar_proc_inst ON apf_hi_varinst (proc_inst_id);
CREATE INDEX apf_idx_hi_procvar_name_type ON apf_hi_varinst (name, var_type);
CREATE INDEX apf_idx_hi_procvar_task_id ON apf_hi_varinst (task_id);

CREATE TABLE apf_ru_identitylink (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    rev INT DEFAULT 0,
    ident_type VARCHAR(255) NULL,
    group_id VARCHAR(255) NULL,
    user_id VARCHAR(255) NULL,
    task_id VARCHAR(255) NULL REFERENCES apf_ru_task(id),
    proc_inst_id VARCHAR(255) NULL REFERENCES apf_ru_execution(id),
    proc_def_id VARCHAR(255) NULL REFERENCES apf_re_procdef(id)
);
CREATE INDEX apf_idx_ident_lnk_user ON apf_ru_identitylink (user_id);
CREATE INDEX apf_idx_ident_lnk_group ON apf_ru_identitylink (group_id);

CREATE TABLE apf_hi_identitylink (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    rev INT DEFAULT NULL,
    ident_type VARCHAR(255) NULL,
    group_id VARCHAR(255) NULL,
    user_id VARCHAR(255) NULL,
    task_id VARCHAR(255) NULL ,
    proc_inst_id VARCHAR(255) NULL,
    proc_def_id VARCHAR(255) NULL REFERENCES apf_re_procdef(id)
);
CREATE INDEX apf_idx_hi_ident_lnk_user ON apf_ru_identitylink (user_id);
CREATE INDEX apf_idx_hi_ident_lnk_group ON apf_ru_identitylink (group_id);
CREATE INDEX apf_idx_hi_ident_lnk_task ON apf_ru_identitylink (task_id);
CREATE INDEX apf_idx_hi_ident_lnk_procinst ON apf_ru_identitylink (proc_inst_id);
CREATE INDEX apf_idx_hi_ident_lnk_procdef ON apf_ru_identitylink (proc_def_id);
