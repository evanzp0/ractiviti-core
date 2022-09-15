-- Add migration script here
-- Add migration script here
drop table if exists apf_hi_identitylink;
drop table if exists apf_ru_identitylink;
drop table if exists apf_hi_varinst;
drop table if exists apf_ru_variable;
drop table if exists apf_hi_taskinst;
drop table if exists apf_ru_task;
drop table if exists apf_hi_procinst;
drop table if exists apf_hi_actinst;
drop table if exists apf_ru_execution;
drop table if exists apf_re_procdef;
drop table if exists apf_ge_bytearray;
drop table if exists apf_re_deployment;

create table apf_re_deployment (
    id varchar(36) not null primary key,
    name varchar(255) null,
    key varchar(255) null,
    organization varchar(255) null,
    deployer varchar(255) null,
    deploy_time timestamp not null default current_timestamp
);

create table apf_re_procdef (
    id varchar(36) not null primary key,
    rev int default null,
    name varchar(255) null,
    key varchar(255)  not null,
    version int not null default 1,
    deployment_id varchar(36) not null references apf_re_deployment(id),
    resource_name varchar(4000) null,
    description varchar(4000) null,
    suspension_state int default 0,

    unique (key, version)
);

create table apf_ge_bytearray (
    id varchar(36) not null primary key,
    name varchar(255) null,
    deployment_id varchar(36) not null references apf_re_deployment(id),
    bytes bytea
);

create table apf_ru_execution (
    id varchar(36) not null primary key,
    rev int default null,
    proc_inst_id varchar(36) null references apf_ru_execution(id),
    business_key varchar(255) null,
    parent_id varchar(36) null references apf_ru_execution(id),
    proc_def_id varchar(36) null references apf_re_procdef(id),
    root_proc_inst_id varchar(36) null,
    element_id varchar(255) null,
    is_active int default 1,
    start_time timestamp not null default current_timestamp,
    start_user varchar(255) null
);

create table apf_hi_actinst (
    id varchar(36) not null primary key,
    rev int default null,
    proc_def_id varchar(36) not null,
    proc_inst_id varchar(36) not null,
    execution_id varchar(36)  not null,
    task_id varchar(36) null,
    element_id varchar(255) not null,
    element_name varchar(255) null,
    element_type varchar(255) null,
    start_user_id varchar(255) null,
    end_user_id varchar(255) null,
    start_time timestamp not null default current_timestamp,
    end_time timestamp null,
    duration bigint null
);
create index apf_idx_hi_act_inst_start on apf_hi_actinst (start_time);
create index apf_idx_hi_act_inst_end on apf_hi_actinst (end_time);
create index apf_idx_hi_act_inst_procinst on apf_hi_actinst (proc_inst_id, element_id);
create index apf_idx_hi_act_inst_exec on apf_hi_actinst (execution_id, element_id);

create table apf_hi_procinst (
    id varchar(36) not null primary key,
    rev int default null,
    proc_inst_id varchar(36) not null,
    business_key varchar(255) null,
    proc_def_id varchar(36) not null,
    start_time timestamp not null default current_timestamp,
    end_time timestamp null,
    duration bigint null default 0,
    start_user varchar(255) null,
    start_element_id varchar(255) null,
    end_element_id varchar(255) null,

    unique (proc_inst_id)
);
create index apf_idx_hi_pro_inst_end on apf_hi_procinst (end_time);
create index apf_idx_hi_pro_i_buskey on apf_hi_procinst (business_key);

create table apf_ru_task (
    id varchar(36) not null primary key,
    rev int default null,
    execution_id varchar(36) not null references apf_ru_execution(id),
    proc_inst_id varchar(36) not null references apf_ru_execution(id),
    proc_def_id varchar(36) not null references apf_re_procdef(id),
    element_id varchar(255) not null,
    element_name varchar(255) null,
    element_type varchar(255) null,
    business_key varchar(255) null,
    description varchar(4000) null,
    start_user_id varchar(255) null,
    create_time timestamp not null default current_timestamp,
    suspension_state int not null default 0,
    form_key varchar(255) null
);
create index apf_idx_task_create on apf_ru_task (create_time);
create index apf_idx_task_exe on apf_ru_task (execution_id);
create index apf_idx_task_procinst on apf_ru_task (proc_inst_id);
create index apf_idx_task_procdef on apf_ru_task (proc_def_id);

create table apf_hi_taskinst (
    id varchar(36) not null primary key,
    rev int default null,
    execution_id varchar(36) not null,
    proc_inst_id varchar(36) not null,
    proc_def_id varchar(36) not null,
    element_id varchar(255) not null,
    element_name varchar(255) null,
    element_type varchar(255) null,
    business_key varchar(255) null,
    description varchar(4000) null,
    start_user_id varchar(255) null,
    end_user_id varchar(255) null,
    start_time timestamp not null,
    end_time timestamp null,
    duration bigint null,
    suspension_state int null,
    form_key varchar(255) null
);
create index apf_idx_hi_task_inst_procinst on apf_hi_taskinst (proc_inst_id);

create table apf_ru_variable (
    id varchar(36) not null primary key,
    rev int default null,
    var_type varchar(255) not null,
    name varchar(255) not null,
    execution_id varchar(36) null,
    proc_inst_id varchar(36) not null references apf_ru_execution(id),
    task_id varchar(36) null , -- 任务结束后，任务会被删除，但是变量需要留着给后续流程使用
    value varchar(4000) null
);
create index apf_idx_variable_task_id on apf_ru_variable (task_id);
create index apf_fk_var_exe on apf_ru_variable (execution_id);
create index apf_fk_var_procinst on apf_ru_variable (proc_inst_id);

create table apf_hi_varinst (
    id varchar(36) not null primary key,
    rev int default null,
    var_type varchar(255) not null,
    name varchar(255) not null,
    execution_id varchar(36) null ,
    proc_inst_id varchar(36) not null,
    task_id varchar(36) null,
    value varchar(4000) null,
    create_time timestamp null,
    last_updated_time timestamp null
);
create index apf_idx_hi_procvar_proc_inst on apf_hi_varinst (proc_inst_id);
create index apf_idx_hi_procvar_name_type on apf_hi_varinst (name, var_type);
create index apf_idx_hi_procvar_task_id on apf_hi_varinst (task_id);

create table apf_ru_identitylink (
    id varchar(36) not null primary key,
    rev int default 0,
    ident_type varchar(255) null,
    group_id varchar(255) null,
    user_id varchar(255) null,
    task_id varchar(36) null references apf_ru_task(id),
    proc_inst_id varchar(36) null references apf_ru_execution(id),
    proc_def_id varchar(36) null references apf_re_procdef(id)
);
create index apf_idx_ident_lnk_user on apf_ru_identitylink (user_id);
create index apf_idx_ident_lnk_group on apf_ru_identitylink (group_id);

create table apf_hi_identitylink (
    id varchar(36) not null primary key,
    rev int default null,
    ident_type varchar(255) null,
    group_id varchar(255) null,
    user_id varchar(255) null,
    task_id varchar(36) null ,
    proc_inst_id varchar(36) null,
    proc_def_id varchar(36) null references apf_re_procdef(id)
);
create index apf_idx_hi_ident_lnk_user on apf_ru_identitylink (user_id);
create index apf_idx_hi_ident_lnk_group on apf_ru_identitylink (group_id);
create index apf_idx_hi_ident_lnk_task on apf_ru_identitylink (task_id);
create index apf_idx_hi_ident_lnk_procinst on apf_ru_identitylink (proc_inst_id);
create index apf_idx_hi_ident_lnk_procdef on apf_ru_identitylink (proc_def_id);
