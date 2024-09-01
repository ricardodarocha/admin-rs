create table if not exists enviar_email (
   id  VARCHAR(40) not null PRIMARY key,
   id_email VARCHAR(40) not null references contato(id),
   body text );

insert into enviar_email values (
'0190ef6e-0612-7f23-98b0-ed146428556d', '0f81ca8e-2981-5f88-aa95-41c8cbc22a68', 'l3mlyp-3uf62y'), --admin
('0190ef4c-2d28-7371-ba3d-2e7c149ac6b0', 'f5cde193-1f51-51ae-9b11-d41930704ab0', 'l3mlyp-3uf62y'), --super
('0190ef5d-796e-7a13-8c73-1d0b1e11d5ba', 'f5cde193-1f51-51ae-9b11-d41930704ab3', '1dpqb5-l6l5cw'); --dev

--senha caze 'l3mlyp-3uf62y' -- '0bd920dcc8b4c400f0b23af358975cc1'
--senha ricardo dev '1dpqb5-l6l5cw' -- '9577b181f6afb2cdae931866b2051fbf'

