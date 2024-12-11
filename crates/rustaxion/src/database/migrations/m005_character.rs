use sea_orm::{ActiveModelTrait, Set};
use sea_orm_migration::{prelude::*, schema::*};

use crate::database;

use super::m002_player::Player;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Character::Table)
                    .if_not_exists()
                    .col(pk_auto(Character::Id))
                    .col(string(Character::Name))
                    .col(string(Character::Description))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Player::Table)
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_player-selected_character_id")
                            .from_tbl(Player::Table)
                            .from_col(Player::SelectedCharacterId)
                            .to_tbl(Character::Table)
                            .to_col(Character::Id),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_player-head_id")
                            .from_tbl(Player::Table)
                            .from_col(Player::HeadId)
                            .to_tbl(Character::Table)
                            .to_col(Character::Id),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        type CharacterModel = database::entities::character::ActiveModel;
        let characters = vec![
            CharacterModel {
                id: Set(10010),
                name: Set("伊莎贝尔".into()),
                description: Set("在新纪元后出生的孩子，成年后马上加入了音灵训练营，每天都在努力地争取早日摘掉菜鸟的称号。哪怕从未踏上过地球，但对抗智能联邦的决心绝不输给任何一位音灵成员。".into()),
            },
            CharacterModel {
                    id: Set(10020),
                    name: Set("阿修贝尔".into()),
                    description: Set("在新纪元后出生的孩子，成年后马上加入了音灵训练营，每天都在努力地争取早日摘掉菜鸟的称号。哪怕从未踏上过地球，但对抗智能联邦的决心绝不输给任何一位音灵成员。".into()),
            },
            CharacterModel {
                    id: Set(10030),
                    name: Set("莲音/Lain".into()),
                    description: Set("“音灵”的主要教官。幼年因先天性失声的缘故饱受歧视，但从未放弃将心中的美好传递给身边的人。在加入“音灵”后为了与大家交流而借助语音助手发（jia）声（chang），，因而其他成员私下里称为“那个违和感强大的女人“。".into()),
            },
            CharacterModel {
                    id: Set(10040),
                    name: Set("欧罗拉/Aurora".into()),
                    description: Set("“音灵”的主要教官，不苟言笑的她很少对他人吐露心声，事实上是个温柔的好女孩。据说还在地球时是豪门千金，“音灵”组织的数艘太空艇也是其家族赞助而来。".into()),
            },
            CharacterModel {
                    id: Set(10050),
                    name: Set("夏尔/Ciel".into()),
                    description: Set("“音灵”的主要教官，在地球时曾是一名小说家，曾为了寻求灵感而环游世界。细心温柔的他在组织里很有人气，因为做事总是一副慢悠悠的样子而遭到欧罗拉的反感，自己却不以为意。".into()),
            },
            CharacterModel {
                    id: Set(10060),
                    name: Set("零/Eins".into()),
                    description: Set("“音灵”的创立者，组织的大脑核心。只有极少有人亲眼见过她，也只有少部分成员有权限通过邮件获取指令。几乎不与人接触的她却对组织大小事无所不知，获取情报的来源未知。".into()),
            },
            CharacterModel {
                    id: Set(20020),
                    name: Set("Zing".into()),
                    description: Set("自诩浮游生物般的漫游者，常常感叹广阔空间里的虚无，但也在尝试用各种古怪的办法自得其乐。意外通过ZYON号与其他个体产生连接后，心态变得积极起来。".into()),
            },
            CharacterModel {
                    id: Set(20030),
                    name: Set("Sin".into()),
                    description: Set("一位充满神秘感的黑长直少女。似乎因为过去曾误伤了自己最爱的亲人而选择了流放自己的道路。".into()),
            },
            CharacterModel {
                    id: Set(20040),
                    name: Set("Zing（幼年）".into()),
                    description: Set("自诩浮游生物般的漫游者，常常感叹广阔空间里的虚无，但也在尝试用各种古怪的办法自得其乐。意外通过ZYON号与其他个体产生连接后，心态变得积极起来。".into()),
            },
            CharacterModel {
                    id: Set(20050),
                    name: Set("眠（幼年）".into()),
                    description: Set("拥有不可思议能量的少女。在宇宙中漂浮了很久，久到忘记了自己的身份和过去，只记得要找到某个人。紧紧抓住唯一的期许，执着地穿行在茫茫宇宙。".into()),
            },
            CharacterModel {
                    id: Set(20060),
                    name: Set("眠".into()),
                    description: Set("拥有不可思议能量的少女。在宇宙中漂浮了很久，久到忘记了自己的身份和过去，只记得要找到某个人。紧紧抓住唯一的期许，执着地穿行在茫茫宇宙中。".into()),
            },
            CharacterModel {
                    id: Set(20090),
                    name: Set("蕾蒂西亚".into()),
                    description: Set("生活在古堡中，远离尘世喧嚣的魔法使。不要问她关于年龄的任何事！最大的乐趣是和妹妹斗嘴。似乎瞧不起所有人。闲暇时经常闷头阅读各类神话传说。".into()),
            },
            CharacterModel {
                    id: Set(20170),
                    name: Set("Zing-233号".into()),
                    description: Set("梓音的备用体型之一，因为可以装嫩(?)所以相当受梓音的喜欢，结果在登上载音号时被弄丢了。哪位好心人要是捡到了，请对天大喊“世界上最可爱的AI是谁？”召唤梓音前往领取。".into()),
            },
            CharacterModel {
                    id: Set(30040),
                    name: Set("AI-仙鹤型".into()),
                    description: Set("不为人知的地下社团成员。虽然只是个柔弱的少女，却满脸都写着”惹不起“。".into()),
            },
            CharacterModel {
                    id: Set(40010),
                    name: Set("飞鱼".into()),
                    description: Set("稚气未脱的棕发少年。比起一般的男孩子，似乎总有些胆怯，但是非常努力，做每件事都很认真。".into()),
            },
            CharacterModel {
                    id: Set(40040),
                    name: Set("莉欧娜".into()),
                    description: Set("射击天赋极高的女孩子，仿佛为狙击而生。有人以为她是冷血的杀手，只有队友知道平时的她有多吵。。。吃个饭都能提出20个方案哦！".into()),
            },
            CharacterModel {
                    id: Set(40090),
                    name: Set("晶".into()),
                    description: Set("出身于占卜师世家的神秘女孩。虽然流着占卜师的血液，却似乎对占卜术有些抵触，从来没有接受过别人的占卜请求。".into()),
            },
            CharacterModel {
                    id: Set(40120),
                    name: Set("朵菲".into()),
                    description: Set("喜欢在海中游泳的女孩子，曾被路过的旅人看错当成是美人鱼。据说使用了最新科技的防晒设备，不管怎么海水浴都不会被晒黑。".into()),
            },
            CharacterModel {
                    id: Set(40150),
                    name: Set("雪秤".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(40250),
                    name: Set("希卡".into()),
                    description: Set("像男孩子一样不拘小节的女生。年纪虽然小，却意外地很会照顾人。她自己说，这是因为家里有个顽皮的弟弟。".into()),
            },
            CharacterModel {
                    id: Set(40320),
                    name: Set("牧人".into()),
                    description: Set("温柔娴静的大姐姐。为人非常谦和低调，但其实在网络世界里是知名的画师。".into()),
            },
            CharacterModel {
                    id: Set(40330),
                    name: Set("双子".into()),
                    description: Set("可爱的呆毛萝莉。虽然看起来小学还没毕业，但有着惊人的卡牌游戏天赋，几乎从来没输过。".into()),
            },
            CharacterModel {
                    id: Set(50010),
                    name: Set("小IA".into()),
                    description: Set("“在世界巡回演出途中，一不小心就迷路了，哎呀太迷糊了~”据说是来自虚拟世界的著名歌姬，既然来都来了，不如就在这里也开一场演唱会吧！水晶般清澈透亮的歌声，在这个全新宇 宙中唱响。".into()),
            },
            CharacterModel {
                    id: Set(30050),
                    name: Set("佑希".into()),
                    description: Set("惑星“亚拉腊”原住居民的他在幼年便被族人流放。在一次“音灵”的军事演习中被燃烧的炮火误伤，阴差阳错被欧罗拉收养。因为“音灵”内部男女失衡，主动成为了女装大佬在星际补给站为星舰巡航者们提供补给，颇受大家的疼爱。".into()),
            },
            CharacterModel {
                    id: Set(40130),
                    name: Set("米拉".into()),
                    description: Set("心里想的跟嘴上说的总会相反，明明喜欢却会说讨厌，所谓的傲娇指的就是米拉这种女孩子吧。在空间站工作，无聊的时候会摆弄手上的数据魔方。".into()),
            },
            CharacterModel {
                    id: Set(40220),
                    name: Set("爱莉".into()),
                    description: Set("亚拉腊人，喜欢游泳，跟朵菲是好朋友。最近在计划和最喜欢的朵菲一起在亚拉腊的繁星海岸边搭个夏日刨冰摊。真正的耳朵是头上的兔耳，机械耳装饰只是个人爱好。".into()),
            },
            CharacterModel {
                    id: Set(30090),
                    name: Set("阿尔塔芙".into()),
                    description: Set("阿尔塔芙会参加竞技场的十二螺旋战完全可以说是被安塔瑞斯威逼利诱来的——如果不能带上自己的宠物蟹蟹，以她的性格就更不会来了。输了会很伤心！赢了别人会很伤心！总会有人得不到幸福，那我为什么要参加这种比赛呢哇哇哇。瘦小的身躯一边这样抹着眼泪，一边把一个个对手甩在身后。".into()),
            },
            CharacterModel {
                    id: Set(30100),
                    name: Set("安塔瑞斯".into()),
                    description: Set("得不到胜利的奖杯，就用毒针刺向我的敌人吧——当然，安塔瑞斯并没有真的长毒针。安塔瑞斯是竞技场的常客，违反常理的战斗方式和敏锐的洞察力让她获得过多次胜利。你可能想象不到这个漂亮的女孩子有多强的好胜心，每次输掉比赛她都会躲到房间用辫子上的勾状发饰不停地戳对手的照片……总之，不要惹她就对了！".into()),
            },
            CharacterModel {
                    id: Set(30110),
                    name: Set("希玛".into()),
                    description: Set("“竞技？除了证明自己的实力之外，当然是找到命中注定的对手啦！”。这位对“宿命般的死敌”有着奇怪憧憬的孩子，在宇宙移动竞技场——伊克利普斯的十二螺旋战中一路连胜，直到因体力不支惜败于某位音灵成员手下，屈居第二名。但被打败的她似乎...更加燃起斗志了呢？\"找到你啦♥\"".into()),
            },
            CharacterModel {
                    id: Set(30070),
                    name: Set("阿德莉（Adeliae）".into()),
                    description: Set("无论什么温度下都穿着厚厚的棉衣和……过膝袜的女孩子。在星际补给站工作，脾气比外表暴躁。不知道是因为过于迟钝还是单纯的呆板，目前还把佑希当作女孩子看待。不可以叫她呆鹅，会被打。".into()),
            },
            CharacterModel {
                    id: Set(40260),
                    name: Set("艾肯".into()),
                    description: Set("看似危险的外星种族少女——实际上见到可爱的孩子会直接扑上去捏人脸蛋。喜欢管所有人叫小孩，因此多次遭到音灵成员抗议。外表年轻，实际年龄是{forbiddenObject}。".into()),
            },
            CharacterModel {
                    id: Set(1006010),
                    name: Set("觉醒零".into()),
                    description: Set("“音灵”的创立者，组织的大脑核心，科学家。极少人亲眼见过她，只有少部分成员有权限通过邮件获取指令，几乎不与人接触的她却对组织大小事无所不知。获取情报的来源未知。 ----“零的能力不容置疑”".into()),
            },
            CharacterModel {
                    id: Set(3004010),
                    name: Set("觉醒AI-仙鹤型".into()),
                    description: Set("“自称”不为人知的地下社团成员。外表看起来只是个柔弱的少女，却总凶巴巴地板着脸。背后长着翅膀，但没见她飞过。本人并不愿透露自己是什么种族、来自哪个星球，连关于“不为 人知的地下社团”都不愿多提几句。".into()),
            },
            CharacterModel {
                    id: Set(4022010),
                    name: Set("爱莉·探海者".into()),
                    description: Set("喜欢在做事时遇到一点偏差就重头做起，导致给周围人的日常生活造成了些许麻烦。爱莉对更深处的海洋充满了无比的好奇心，但因某次巨型乌鲁的袭击在她心中埋下了阴影；在她不敢潜入深海的日子里，最喜欢的事就是和朵菲在浅滩上嬉戏，教朵菲如何适应亚拉腊的海，还有听朵菲讲人类的事情。".into()),
            },
            CharacterModel {
                    id: Set(50050),
                    name: Set("梅兹".into()),
                    description: Set("音灵医疗部二等医护官。虽然很低调，但确实是个天才。喜欢各种充满冒险与幻想的故事，夏尔偶尔发布在音灵公共论坛上的短篇冒险小说是她活命的根源。作为一位冒险故事爱好者，这人却是位理直气壮的胆小鬼，或者说，对自己的实力非常地有自知之明？".into()),
            },
            CharacterModel {
                    id: Set(30010),
                    name: Set("薇拉".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(30020),
                    name: Set("克拉特".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(30030),
                    name: Set("德菲娜斯".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(5005010),
                    name: Set("梅兹·医护员".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(30060),
                    name: Set("天鹰".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(40050),
                    name: Set("琴女".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(30080),
                    name: Set("天蝶".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(40080),
                    name: Set("钟灵".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(40160),
                    name: Set("波利佳".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(50020),
                    name: Set("比比".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(50030),
                    name: Set("佑乃".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(50040),
                    name: Set("哈蕾娜".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(5003010),
                    name: Set("佑乃·探险者".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(40170),
                    name: Set("茜茜".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(40210),
                    name: Set("希卡".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(40310),
                    name: Set("安途".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(40060),
                    name: Set("九头蛇女".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(50080),
                    name: Set("贝勒米".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(50060),
                    name: Set("忒斯提".into()),
                    description: Set("".into()),
            },
            CharacterModel {
                    id: Set(50070),
                    name: Set("紫椋".into()),
                    description: Set("".into()),
            }
        ];

        for char in characters {
            char.insert(db).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Player::Table)
                    .drop_foreign_key(Alias::new("fk_player-selected_character_id"))
                    .drop_foreign_key(Alias::new("fk_player-head_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Character::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Character {
    Table,
    Id,
    Name,
    Description,
}
